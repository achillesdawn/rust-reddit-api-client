use reddit::{api::Kind, async_client::Reddit};
use tracing::{error, info, warn};

async fn following_user_posts() -> eyre::Result<()> {
    let mut reddit = Reddit::new().await?;

    let profiles = reddit.following(Some(10)).await?;

    let mut profiles = profiles
        .iter()
        .filter_map(Kind::as_profile)
        .collect::<Vec<_>>();

    profiles.sort_by(|a, b| b.created.total_cmp(&a.created));

    let mut join_set = tokio::task::JoinSet::new();

    for profile in profiles {
        if !profile.display_name.starts_with("u_") {
            warn!(profile = profile.display_name, "skipping");
            continue;
        }

        let user = &profile.display_name[2..];

        let mut downloaded = 0u32;

        let items = match reddit
            .user_latest(
                user,
                reddit::api::enums::Endpoint::Submitted,
                reddit::api::enums::SortType::New,
                None,
                None,
            )
            .await
        {
            Ok(items) => items,
            Err(err) => {
                error!(?err);
                continue;
            }
        };

        let items = items
            .data
            .children
            .iter()
            .filter_map(Kind::as_post)
            .collect::<Vec<_>>();

        info!(user, num_posts = items.len());

        if items.is_empty() {
            warn!(user, "NO POSTS");
            continue;
        }

        for item in items {
            join_set.spawn(reddit::async_client::get_post_images(*item));
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

    info!("DONE");

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    dotenv::from_filename("ghost.env").unwrap();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Err(err) = following_user_posts().await {
                error!("{}", err);
            }
        });
}
