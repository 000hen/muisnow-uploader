use std::{time::SystemTime, fmt, io, env, path::PathBuf};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use openssl::hash::DigestBytes;

#[derive(PartialEq, Debug)]
pub enum HashType {
    Fingerprint,
    Deafult
}

pub struct IdentifyString {
    pub identify: String,
    pub timestamp: u64
}

pub fn create_identify_string(action: &str, data: &str) -> IdentifyString {
    let current_time = SystemTime::now();

    let secs = current_time.duration_since(SystemTime::UNIX_EPOCH).expect("Cannot get current time").as_secs();

    IdentifyString {
        identify: vec![action, data, &secs.to_string()].join(","),
        timestamp: secs
    }
}

pub fn digest_bytes_to_hash(bytes: DigestBytes, output_type: Option<HashType>) -> String {
    let string_type = match output_type {
        Some(hash_type) => hash_type,
        None => HashType::Deafult
    };

    let bytes = bytes.iter();

    if string_type == HashType::Fingerprint {
        bytes.map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(":")
    } else {
        bytes.map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
}

pub fn create_file_progress_bar(display: &str, size: u64) -> ProgressBar {
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::with_template(&vec![display, " {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"].join(""))
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    pb
}

pub fn get_current_dir() -> io::Result<PathBuf> {
    let mut path = env::current_exe()?;
    path.pop();

    Ok(path)
}

pub fn get_dir_path(dir: &str) -> io::Result<PathBuf> {
    let mut path = get_current_dir()?;
    path.push(dir);

    Ok(path)
}