mod api;
mod async_client;
mod token;

use async_client::Reddit;

fn main() {
    dotenv::from_filename(".env").unwrap();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut reddit = Reddit::new();
            reddit.authorize().await.unwrap();
            // reddit.subreddit("blender").await.unwrap();

            let followers = vec![
                "AskEconomics",
                "econmonitor",
                "Economics",
                "economy",
                "finance",
                "GameDeals",
            ];

            let followers: Vec<&str> = followers
                .into_iter()
                .filter(|item| item.starts_with("u/"))
                .map(|item| item.split_once("u/").unwrap().1)
                .collect();
            dbg!(&followers);

            let mut join_set = tokio::task::JoinSet::new();

            for follower in followers {
                println!("getting {}", follower);
                let posts = match reddit.user_profile(follower.to_owned()).await {
                    Ok(posts) => posts,
                    Err(err) => {
                        dbg!(err);
                        continue;
                    },
                };

                for post in posts {
                    join_set.spawn(async_client::get_post_images(post));
                }
            }

            while let Some(res) = join_set.join_next().await {
                dbg!(res);
            }
        });
}
