use std::{io::Result, cmp::min, path::Path};

use async_stream::stream;
use indicatif::HumanBytes;
use reqwest::{header::HeaderMap, Client, multipart::Form, Body, multipart::Part};
use tokio::fs::File;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, BytesCodec};

use crate::{config::get_config, utils::create_file_progress_bar};

#[derive(Deserialize)]
struct UploadDetail {
    path: String,
    host: String
}

#[derive(Deserialize)]
struct UploadResponse {
    data: UploadDetail
}

pub async fn send_request(fingerprint: &str, request_type: &str, signature: &str, timestamp: u64, file_path: &str) -> Result<String> {
    let config = get_config().clone();
    let host = env!("UPLOAD_HOST");
    let protocol = env!("UPLOAD_PROTOCOL");
    let combine = format!("{}://{}/api/staff", protocol, host);

    let mut header = HeaderMap::new();

    header.insert("a-pk-fingerprint", fingerprint.parse().unwrap());
    header.insert("a-type", request_type.parse().unwrap());
    header.insert("a-signature", signature.parse().unwrap());
    header.insert("user-agent", "Muisnow-Uploader".parse().unwrap());

    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let max_size = metadata.len();
    let filename = String::from(Path::new(file_path).file_name().unwrap().to_str().unwrap());

    if !config.invisible {
        println!();
        println!("@HTTP INFORMATION");
        println!();
        println!("Request URL: {}", combine);
        println!("Send file: {}", file_path);
        println!("Filename: {}", &filename);
        println!("File Size: {}", HumanBytes(max_size));
        println!();
    }
    
    let mut uploaded = 0;

    let mut reader_stream = FramedRead::with_capacity(file, BytesCodec::new(), 1024 * 1024);

    let pb = match !config.invisible {
        true => Some(create_file_progress_bar("Uploading", max_size)),
        false => None
    };
    
    let stream = stream! {
        while let Some(chunk) = reader_stream.next().await {
            if let Ok(chunk) = &chunk {
                // println!("Chunk Size: {}", chunk.len());
                let new = min(uploaded + (chunk.len() as u64), max_size);
                uploaded = new;

                if let Some(progress) = &pb {
                    progress.set_position(new);

                    if uploaded >= max_size {
                        progress.finish_and_clear();
                    }
                }
            }
            yield chunk;
        }
    };

    let multipart = Form::new()
        .text("a-timestamp", timestamp.to_string())
        .part("a-file", Part::stream(Body::wrap_stream(stream)).file_name(filename).mime_str("application/octet-stream").unwrap());

    let client = Client::new();
    let request: UploadResponse = client.post(combine)
        .headers(header)
        .multipart(multipart)
        .send()
        .await
        .unwrap()
        .json::<UploadResponse>()
        .await
        .unwrap();

    Ok(format!("{}{}", request.data.host, request.data.path))
}