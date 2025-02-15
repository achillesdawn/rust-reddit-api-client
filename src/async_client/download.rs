use std::{io::Write, path::PathBuf, str::FromStr};

use crate::api::Post;

struct Media {
    id: String,
    extension: String,
}

async fn download(url: String, path: PathBuf) {
    println!("{}->{:?}", url, path);

    let res = reqwest::get(url).await.unwrap();
    let bytes = res.bytes().await.unwrap();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

fn prepare_output_path(post: &Post) -> (PathBuf, bool) {
    let output_dir = PathBuf::from_str("assets").unwrap();
    let mut output_path = std::path::absolute(output_dir).unwrap();

    output_path.push(&post.author);
    if post.title.len() > 255 {
        let name: String = post.title.chars().take(200).collect();
        output_path.push(name);
    } else {
        output_path.push(&post.title);
    }

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

pub async fn get_post_images(post: Post) {
    let (output_path, _exists) = prepare_output_path(&post);

    let mut join_set = tokio::task::JoinSet::new();

    let mut media: Vec<Media> = Vec::new();

    if let Some(metadata) = post.media_metadata {
        for (_, media_meta) in metadata.into_iter() {
            let (Some(media_type), Some(id)) = (media_meta.m, media_meta.id) else {
                continue;
            };

            let extension: &str;

            match media_type {
                x if x.contains("gif") => {
                    media.push(Media {
                        id,
                        extension: "gif".to_owned(),
                    });

                    continue;
                }
                x if x.contains("jpg") => {
                    extension = "jpg";
                }
                x if x.contains("png") => {
                    extension = "png";
                }
                _ => {
                    let message = format!("unhandled media type: {}", media_type);
                    dbg!(message);
                    panic!("unhandled media type");
                }
            }

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

        println!("{} -> {:?}", url, image_path);

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
        if res.is_err() {
            dbg!(res.err());
        }
    }
}
