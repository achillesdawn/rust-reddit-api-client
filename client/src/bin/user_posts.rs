use std::collections::HashMap;

use reddit::async_client::Reddit;
use tracing::{error, info, warn};

async fn get_posts() {
    let mut reddit = Reddit::new().await.unwrap();

    info!("authorized");

    let mut profiles = reddit.following().await.unwrap();

    profiles.sort_by(|a, b| b.created.total_cmp(&a.created));

    let mut join_set = tokio::task::JoinSet::new();

    for profile in profiles {
        if !profile.display_name.starts_with("u_") {
            warn!(profile = profile.display_name, "skipping");
            continue;
        }

        let user = &profile.display_name[2..];

        let mut downloaded = 0u32;

        let posts = match reddit
            .user_latest(reddit::api::Endpoint::Submitted, user, None)
            .await
        {
            Ok(posts) => posts,
            Err(err) => {
                error!(?err);
                continue;
            }
        };

        info!(user, num_posts = posts.len());

        if posts.is_empty() {
            warn!(user, "NO POSTS");
            continue;
        }

        for post in posts {
            join_set.spawn(reddit::async_client::get_post_images(post));
        }

        if let Some(join) = join_set.join_next().await {
            downloaded += match join {
                Ok(d) => d,
                Err(err) => {
                    error!(?err, "join error");
                    continue;
                }
            }
        }

        info!(downloaded, user);
    }

    info!("DONE")
}

async fn related_subreddits() {
    let mut reddit = Reddit::new().await.unwrap();

    let posts = reddit.subreddit_posts_latest("blender").await.unwrap();

    let mut users = Vec::new();

    for post in posts {
        users.push(post.author);
    }

    let mut counts: HashMap<String, u32> = HashMap::new();

    for user in users.into_iter() {
        let user_posts = reddit.user(&user, None).await.unwrap();
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
    let mut reddit = Reddit::new().await.unwrap();

    let posts = match reddit
        .user_latest(
            reddit::api::Endpoint::Submitted,
            "Individual_Air_5532",
            None,
        )
        .await
    {
        Ok(posts) => posts,
        Err(err) => {
            let _ = dbg!(err);
            return;
        }
    };

    let mut join_set = tokio::task::JoinSet::new();

    for post in posts {
        join_set.spawn(reddit::async_client::get_post_images(post));
    }

    while let Some(res) = join_set.join_next().await {
        if res.is_err() {
            dbg!(res.err());
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    dotenv::from_filename("ghost.env").unwrap();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(
            get_posts(), // user_profile(),
        );
}
