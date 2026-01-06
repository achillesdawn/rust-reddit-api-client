use std::{io::Write, path::PathBuf, str::FromStr};

use eyre::Context;
use tracing::{debug, error};

use crate::api::Post;

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

    if let Some(metadata) = post.media_metadata {
        for (_, media_meta) in metadata.into_iter() {
            let (Some(media_type), Some(id)) = (media_meta.m, media_meta.id) else {
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
                x if x.contains("jpg") => "jpg",
                x if x.contains("png") => "png",
                _ => {
                    let message = format!("unhandled media type: {}", media_type);
                    dbg!(message);
                    panic!("unhandled media type");
                }
            };

            let Some(source) = media_meta.s.and_then(|item| item.u) else {
                continue;
            };

            let image_path = output_path.join(&id);
            let image_path = image_path.with_extension(extension);

            join_set.spawn(download(source, image_path));
        }
    } else if let Some(preview) = post.preview {
        for image in preview.images {
            let image_path = output_path.join(&image.id);

            join_set.spawn(download(image.source.url, image_path));
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
