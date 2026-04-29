
struct Reddit {
    headers: HashMap<String, String>,
    token: Token,
    base_url: String,
}

impl Reddit {
    fn new() -> Self {
        let headers = HashMap::from_iter([
            (
                "User-Agent".to_owned(),
                "Rust: trends v0.1.0 by u/molivo10".to_owned(),
            ),
            ("Accept-Encoding".to_owned(), "gzip, deflate, br".to_owned()),
            ("Accept".to_owned(), "*/*".to_owned()),
        ]);

        Reddit {
            headers,
            token: Token::new(),

            base_url: "https://oauth.reddit.com".to_owned(),
        }
    }

    fn update_headers(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    fn new_request(&self, url: String, http_request: HttpVerb) -> ureq::Request {
        let mut req = ureq::request(&http_request.to_string(), &url);

        for (key, value) in self.headers.iter() {
            req = req.set(key, value);
        }

        req
    }

    fn authorize(&mut self) -> Result<(), RedditError> {
        let (Ok(client_id), Ok(client_secret), Ok(username), Ok(password)) = (
            env::var("REDDIT_CLIENT_ID"),
            env::var("REDDIT_CLIENT_SECRET"),
            env::var("REDDIT_USERNAME"),
            env::var("REDDIT_PASSWORD"),
        ) else {
            return Err(RedditError::NoEnvVariables);
        };

        let mut req = self.new_request(
            "https://www.reddit.com/api/v1/access_token".to_owned(),
            HttpVerb::POST,
        );

        let encoded = BASE64_STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));
        let authorization = format!("Basic {}", encoded);

        req = req.set("Authorization", &authorization);

        let res = req
            .send_form(&[
                ("grant_type", "password"),
                ("username", &username),
                ("password", &password),
            ])
            .map_err(Box::new)?;

        let token: token::Token = serde_json::from_reader(res.into_reader())?;

        self.update_headers(
            "Authorization".to_owned(),
            format!("Bearer {}", token.access_token),
        );

        self.token = token;

        Ok(())
    }

    fn user_profile(&self, username: String) -> Result<Vec<Post>, RedditError> {
        let url = format!("/user/{username}/submitted");

        let full_url = self.base_url.clone() + &url;

        let mut result = Vec::new();

        let mut query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("context".to_owned(), "2".to_owned()),
            ("show".to_owned(), "given".to_owned()),
            ("sort".to_owned(), "new".to_owned()),
            ("t".to_owned(), "all".to_owned()),
            ("type".to_owned(), "all".to_owned()),
            ("raw_json".to_owned(), "1".to_owned()),
        ]);

        loop {
            let req = self.new_request(full_url.clone(), HttpVerb::GET);

            let res = req
                .query_pairs(query.iter().map(|(k, v)| (k.as_str(), v.as_str())))
                .call()
                .map_err(Box::new)?;

            let deserializer = &mut serde_json::Deserializer::from_reader(res.into_reader());
            let data: api::RedditApiResonse = match serde_path_to_error::deserialize(deserializer) {
                Ok(data) => data,
                Err(err) => {
                    dbg!(&err);
                    let path = err.path().to_string();
                    dbg!(path);
                    return Err(RedditError::NoEnvVariables);
                }
            };

            result.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else {
                dbg!(&data.data.after);
                let after = data.data.after.as_str().unwrap().to_owned();
                query.insert("after".to_owned(), after.clone());
            }
        }

        Ok(result)
    }

    fn subreddit(&self, subreddit_name: &str) -> Result<(), RedditError> {
        let url = format!("/r/{subreddit_name}/new");
        let full_url = self.base_url.clone() + &url;

        let req = self.new_request(full_url, HttpVerb::GET);

        let query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("show".to_owned(), "all".to_owned()),
        ]);

        let res = req
            .query_pairs(query.iter().map(|(k, v)| (k.as_str(), v.as_str())))
            .call()
            .map_err(Box::new)?;

        dbg!(&res);

        let data: RedditApiResonse = serde_json::from_reader(res.into_reader()).unwrap();

        dbg!(data);

        Ok(())
    }

    fn following(&self) -> Result<(), RedditError> {
        let url = format!("/subreddits/mine/subscriber");
        let full_url = self.base_url.clone() + &url;

        let query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("show".to_owned(), "all".to_owned()),
        ]);

        let req = self.new_request(full_url, HttpVerb::GET);

        let res = req
            .query_pairs(query.iter().map(|(k, v)| (k.as_str(), v.as_str())))
            .call()
            .map_err(Box::new)?;

        dbg!(&res);

        let data: serde_json::Value = serde_json::from_reader(res.into_reader()).unwrap();
        let file = std::fs::File::create("result.json").unwrap();

        serde_json::to_writer(file, &data).unwrap();

        Ok(())
    }

    fn download_image(&self, image_link: String) {}
}

fn main() {
    dotenv::from_filename(".env").unwrap();

    let timer = Instant::now();

    let mut r = Reddit::new();
    r.authorize().unwrap();

    // r.subreddit("blender").unwrap();



    let posts = match r.user_profile(followers[0].to_owned()) {
        Ok(posts) => posts,
        Err(err) => {
            dbg!(err);
            std::process::exit(1);
        }
    };

    for post in posts {
        if let Some(preview) = post.preview {
            for image in preview.images {
                dbg!(image.source);
            }
        }
        break;
    }

    // r.following().unwrap();

    let timer = timer.elapsed().as_millis();
    println!("{}ms", timer);
}
