use std::{collections::HashMap, io::Write, path::PathBuf, str::FromStr};

use eyre::Context;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::json;
use tracing::debug;

pub struct ImageClient {
    client: reqwest::Client,
}

impl ImageClient {
    pub fn new() -> Self {
        let headers = json!({
          "accept": "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
          "accept-language": "en-US,en;q=0.9",
          "cache-control": "no-cache",
          "pragma": "no-cache",
          "priority": "u=2, i",
          "sec-ch-ua": "\"Google Chrome\";v=\"141\", \"Not?A_Brand\";v=\"8\", \"Chromium\";v=\"141\"",
          "sec-ch-ua-mobile": "?0",
          "sec-ch-ua-platform": "\"Linux\"",
          "sec-fetch-dest": "image",
          "sec-fetch-mode": "no-cors",
          "sec-fetch-site": "cross-site",
          "sec-fetch-storage-access": "none",
          "Referer": "https://www.reddit.com/",
          "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36",
        });

        let headers: HashMap<String, String> = serde_json::from_value(headers).unwrap();

        let mut header_map = HeaderMap::new();

        for (key, value) in headers.into_iter() {
            let value = HeaderValue::from_str(&value).unwrap();

            let header_key = HeaderName::from_str(&key).unwrap();

            header_map.insert(header_key, value);
        }

        let client = reqwest::ClientBuilder::new()
            .default_headers(header_map)
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn download(&self, url: String, path: PathBuf) -> eyre::Result<bool> {
        if path.exists() {
            return Ok(false);
        }
        debug!(?url, ?path);

        let res = self
            .client
            .get(url)
            .send()
            .await
            .wrap_err("could not get url")?;

        let bytes = res
            .bytes()
            .await
            .wrap_err("could not read response bytes")?;

        let mut file = std::fs::File::create(path).wrap_err("could not create file")?;

        file.write_all(&bytes).wrap_err("could not write to file")?;

        Ok(true)
    }
}
