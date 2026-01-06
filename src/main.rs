mod api;
mod async_client;
mod token;

use std::collections::HashMap;

use async_client::Reddit;
use tracing::{Level, info, warn};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

async fn get_posts() {
    let mut reddit = Reddit::new();
    reddit.authorize().await.unwrap();

    info!("authorized");
    // reddit.subreddit("blender").await.unwrap();

    let profiles = reddit.following().await.unwrap();

    let mut join_set = tokio::task::JoinSet::new();

    for profile in profiles {
        if !profile.display_name.starts_with("u_") {
            continue;
        }

        let follower = &profile.display_name[2..];

        let posts = match reddit.user_profile_latest(follower.to_owned()).await {
            Ok(posts) => posts,
            Err(err) => {
                dbg!(err);
                continue;
            }
        };

        info!(follower, num_posts = posts.len());

        if posts.is_empty() {
            warn!(follower, "NO POSTS");
            continue;
        }

        for post in posts {
            join_set.spawn(async_client::get_post_images(post));
        }
    }

    while let Some(res) = join_set.join_next().await {
        if res.is_err() {
            dbg!(res.err());
        }
    }

    info!("DONE")
}

async fn related_subreddits() {
    let mut reddit = Reddit::new();
    reddit.authorize().await.unwrap();

    let posts = reddit.subreddit("blender").await.unwrap();

    let mut users = Vec::new();

    for post in posts.data.children {
        users.push(post.data.author);
    }

    let mut counts: HashMap<String, u32> = HashMap::new();

    for user in users.into_iter() {
        let user_posts = reddit.user_profile(user).await.unwrap();
        for post in user_posts {
            counts
                .entry(post.subreddit)
                .and_modify(|e| *e += post.ups)
                .or_insert(post.ups);
        }

        dbg!(&counts);
    }
}

async fn user_profile() {
    let mut reddit = Reddit::new();
    reddit.authorize().await.unwrap();

    let posts = match reddit
        .user_profile_latest("Individual_Air_5532".to_owned())
        .await
    {
        Ok(posts) => posts,
        Err(err) => {
            dbg!(err);
            return;
        }
    };

    let mut join_set = tokio::task::JoinSet::new();

    for post in posts {
        join_set.spawn(async_client::get_post_images(post));
    }

    while let Some(res) = join_set.join_next().await {
        if res.is_err() {
            dbg!(res.err());
        }
    }
}

fn init_tracing() {
    let target = Targets::new().with_target(env!("CARGO_PKG_NAME"), Level::DEBUG);

    let timer = tracing_subscriber::fmt::time::ChronoLocal::new("%H:%M:%S%.3f".to_owned());

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().with_timer(timer))
        .with(target)
        .init();
}

fn main() {
    init_tracing();

    dotenv::from_filename("ghost.env").unwrap();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(
            get_posts(), // user_profile(),
        );
}
