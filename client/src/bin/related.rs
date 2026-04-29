use reddit::async_client::Reddit;
use tracing::error;

async fn main_async() -> eyre::Result<()> {
    let mut client = Reddit::new().await?;

    let posts = client.subreddit_posts_latest("selfhosted").await?;

    dbg!(posts);

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
