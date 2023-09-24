#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::io::Result;
use http_worker::send_request;
use openssl::base64;
use sign_worker::{calc_file_hash, sign_buffer, get_public_key_fingerprint};
use utils::{digest_bytes_to_hash, create_identify_string};

use crate::config::get_config;

pub mod utils;
pub mod sign_worker;
pub mod config;
pub mod http_worker;

pub async fn upload(path: &str) -> Result<()> {
    let visible = !get_config().invisible;

    let file_hash = digest_bytes_to_hash(calc_file_hash(path)?, None);
    let make_str = create_identify_string("UPLOAD", &file_hash);
    let sign = base64::encode_block(&sign_buffer(make_str.identify.as_bytes())?);

    if visible {
        println!("---[UPLOAD INFORMATION]---");
        println!("File {}", path);
        println!("Hash: {}", &file_hash);
        println!("Str 2 signature: {}", make_str.identify);
        println!("Signature:");
        println!("{}", sign);
    }

    let file_loc = send_request(&get_public_key_fingerprint()?, "UPLOAD", &sign, make_str.timestamp, path).await?;
    println!("{}", file_loc);

    Ok(())
}