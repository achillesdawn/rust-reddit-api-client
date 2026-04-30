use reddit::async_client::Reddit;
use tracing::error;

async fn main_async() -> eyre::Result<()> {
    let mut client = Reddit::new().await?;

    let posts = client.subreddit_posts_latest("selfhosted").await?;

    let users = posts.iter().map(|p| p.author.as_str()).collect::<Vec<_>>();

    for user in users.into_iter() {
        if let Ok(_items) = client
            .user_latest(
                user,
                reddit::api::enums::Endpoint::Submitted,
                reddit::api::enums::SortType::New,
                None,
                None,
            )
            .await
        {}
    }

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
