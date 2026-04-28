use std::{io::Write, path::PathBuf, str::FromStr, sync::Arc};

use eyre::Context;
use tracing::{debug, error, info, warn};

use crate::{api::Post, async_client::gif_client::RedGifClient};

use super::image_client::ImageClient;

#[derive(Debug)]
struct Media {
    id: String,
    extension: String,
}

async fn download(url: String, path: PathBuf) -> eyre::Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    debug!(?url, ?path);

    let res = reqwest::get(url).await.wrap_err("could not get url")?;

    let bytes = res
        .bytes()
        .await
        .wrap_err("could not read response bytes")?;

    let mut file = std::fs::File::create(path).wrap_err("could not create file")?;

    file.write_all(&bytes).wrap_err("could not write to file")?;

    Ok(true)
}

fn prepare_output_path(post: &Post) -> (PathBuf, bool) {
    let output_dir = PathBuf::from_str("assets").unwrap();
    let mut output_path = std::path::absolute(output_dir).unwrap();

    output_path.push(&post.author);

    let title: String = post
        .title
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .take(200)
        .collect();

    let title = title.replace(" ", "_");

    output_path.push(title.trim());

    let mut exists = false;

    if output_path.exists() {
        exists = true;
    } else {
        match std::fs::create_dir_all(&output_path) {
            Ok(_) => {}
            Err(err) => {
                dbg!(&output_path);
                dbg!(&err);
            }
        };
    }

    (output_path, exists)
}

pub async fn get_post_images(post: Post) -> u32 {
    let (output_path, _exists) = prepare_output_path(&post);

    let mut join_set = tokio::task::JoinSet::new();

    let mut media: Vec<Media> = Vec::new();

    let mut downloaded = 0u32;

    let image_client = Arc::new(ImageClient::new());

    let mut gif_client = RedGifClient::new();

    let post_c = post.clone();

    if let Some(metadata) = post.media_metadata {
        for (_image_id, media_meta) in metadata.into_iter() {
            let (Some(media_type), Some(id)) = (media_meta.m, media_meta.id) else {
                warn!("media has not media type metadata");
                continue;
            };

            let extension = match media_type {
                x if x.contains("gif") => {
                    media.push(Media {
                        id,
                        extension: "gif".to_owned(),
                    });

                    continue;
                }
                x if x.contains("jpg") || x.contains("jpeg") => "jpg",
                x if x.contains("png") => "png",
                _ => {
                    let message = format!("unhandled media type: {}", media_type);
                    dbg!(message);
                    panic!("unhandled media type");
                }
            };

            let Some(source) = media_meta.s.and_then(|item| item.u) else {
                warn!("could not get image source");
                continue;
            };

            let image_path = output_path.join(&id);
            let image_path = image_path.with_extension(extension);
            let image_client_clone = image_client.clone();

            join_set.spawn(async move { image_client_clone.download(source, image_path).await });
        }
    } else if let Some(preview) = post.preview {
        if post.domain == "i.redd.it" {
            for image in preview.images {
                let image_path = output_path.join(&image.id);

                let client_clone = image_client.clone();

                join_set.spawn(
                    async move { client_clone.download(image.source.url, image_path).await },
                );
            }
        } else if post.domain.contains("redgif")
            && let Some((_, video_id)) = post.url.split_once("/watch/")
        {
            let video_path = output_path.join(video_id).with_extension("mp4");

            if !video_path.exists() {
                info!(post.url, video_id, ?video_path, "downloading gif");

                if let Err(err) = gif_client.download(video_id, video_path).await {
                    error!(?err);
                }
            }
        } else if post.domain.contains("i.redd.it") {
            let image_path = output_path.join(&post.name);

            let client_clone = image_client.clone();

            join_set.spawn(async move { client_clone.download(post.url, image_path).await });
        } else if post.domain.contains("v.redd.it") {
        } else {
            dbg!(&preview, &post_c);
        }
    }

    for media in media.into_iter() {
        let image_path = output_path.join(&media.id);

        let url = format!("https://i.redd.it/{}.{}", media.id, media.extension);

        let image_path = image_path.with_extension(media.extension);
        if image_path.exists() {
            continue;
        }

        join_set.spawn(download(url, image_path));
    }

    if let Some(video) = post.media.and_then(|m| m.reddit_video) {
        let mut video_path = output_path.clone();
        video_path.push(output_path.with_extension(".mp4").file_name().unwrap());

        if !video_path.exists() {
            join_set.spawn(download(video.fallback_url, video_path));
        }
    }

    while let Some(res) = join_set.join_next().await {
        let r = match res {
            Ok(i) => i,
            Err(err) => {
                error!(?err, "join error");
                continue;
            }
        };

        let success = match r {
            Ok(s) => s,
            Err(err) => {
                error!(?err, "download error");
                continue;
            }
        };

        if success {
            downloaded += 1;
        }
    }

    downloaded
}
