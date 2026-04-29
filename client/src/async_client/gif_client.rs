use std::path::PathBuf;

use eyre::Context;
use tracing::debug;

pub struct RedGifClient {
    semaphore: tokio::sync::Semaphore,
}

impl RedGifClient {
    pub fn new() -> Self {
        let semaphore = tokio::sync::Semaphore::new(5);
        Self { semaphore }
    }

    fn create_url(video_id: &str) -> String {
        format!("https://api.redgifs.com/v2/gifs/{}/hd.m3u8", video_id)
    }

    pub async fn download(&mut self, video_id: &str, path: PathBuf) -> eyre::Result<bool> {
        // let url = RedGifClient::create_url(video_id);

        let permit = self
            .semaphore
            .acquire()
            .await
            .wrap_err("could not acquire semaphore")?;

        debug!(?path);

        // let mut child = tokio::process::Command::new("ffmpeg")
        //     .args(["-i", &url, "-c", "copy", path.to_str().unwrap()])
        //     .spawn()
        //     .wrap_err("could not spawn process")?;

        // child.wait().await.wrap_err("child exited with error")?;

        drop(permit);

        Ok(true)
    }
}
