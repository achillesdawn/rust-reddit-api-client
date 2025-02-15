use std::{io::Write, path::PathBuf, str::FromStr};

use reqwest::IntoUrl;

use crate::api::Post;

struct Media {
    id: String,
    extension: String,
}

async fn download(url: impl IntoUrl, path: PathBuf) {
    let res = reqwest::get(url).await.unwrap();
    let bytes = res.bytes().await.unwrap();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

pub async fn get_post_images(post: Post) {
    let output_dir = PathBuf::from_str("assets").unwrap();
    let mut output_dir = std::path::absolute(output_dir).unwrap();

    output_dir.push(&post.author);
    if post.title.len() > 255 {
        let name: String = post.title.chars().take(200).collect();
        output_dir.push(name);
    } else {
        output_dir.push(&post.title);
    }

    if output_dir.exists() {
        // return;
    } else {
        match std::fs::create_dir_all(&output_dir) {
            Ok(_) => {}
            Err(err) => {
                dbg!(&output_dir);
                dbg!(&err);
            }
        };
    }

    dbg!(&post.title);

    let mut join_set = tokio::task::JoinSet::new();

    if let Some(gallery) = post.gallery_data {
        for item in gallery.items {
            item.media_id;
        }
    }

    let mut media: Vec<Media> = Vec::new();

    if let Some(metadata) = post.media_metadata {
        for (_, media_meta) in metadata.into_iter() {
            let (Some(media_type), Some(id)) = (media_meta.m, media_meta.id) else {
                continue;
            };

            if media_type.contains("gif") {
                media.push(Media {
                    id,
                    extension: "gif".to_owned(),
                });
            } else if media_type.contains("jpg") {
                if let Some(source) = media_meta.s.and_then(|item| item.u) {
                    let mut image_path = output_dir.clone();
                    image_path.push(&id);
                    let image_path = image_path.with_extension("jpg");

                    join_set.spawn(download(source, image_path));
                }
            } else if media_type.contains("png") {
                if let Some(source) = media_meta.s.and_then(|item| item.u) {
                    let mut image_path = output_dir.clone();
                    image_path.push(&id);
                    let image_path = image_path.with_extension("png");

                    join_set.spawn(download(source, image_path));
                }
            } else {
                let message = format!("unhandled media type: {}", media_type);
                dbg!(message);
                panic!("unhandled media type");
            }
        }
    }

    for media in media.into_iter() {
        let mut image_path = output_dir.clone();
        image_path.push(&media.id);

        let url = format!("https://i.redd.it/{}.{}", media.id, media.extension);

        let image_path = image_path.with_extension(media.extension);
        if image_path.exists() {
            continue;
        }

        println!("{} -> {:?}", url, image_path);

        join_set.spawn(download(url, image_path));
    }

    if let Some(video) = post.media.and_then(|m| m.reddit_video) {
        dbg!(&video);
        let mut video_path = output_dir.clone();
        video_path.push(output_dir.with_extension(".mp4").file_name().unwrap());

        dbg!(&video_path);

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
