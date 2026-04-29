use std::collections::HashMap;

use reddit::async_client::Reddit;
use tracing::error;

async fn main_async() -> eyre::Result<()> {
    let mut client = Reddit::new().await?;

    let posts = client.subreddit_posts_latest("selfhosted").await?;

    let users = posts.iter().map(|p| p.author.as_str()).collect::<Vec<_>>();

    let mut subreddits = HashMap::new();

    for user in users.into_iter() {
        if let Ok(posts) = client
            .user_latest(user, reddit::api::enums::Endpoint::Submitted, None)
            .await
        {
            for post in posts {
                let entry = subreddits.entry(post.subreddit).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut v = subreddits.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|i| i.1);

    dbg!(v);

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("could not build tokio runtime")
        .block_on(async {
            if let Err(err) = main_async().await {
                error!("{:?}", err);
            }
        })
}
