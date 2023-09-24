use std::{path::Path, io::{Result, Write, Read}, fs::{File, self}};

use openssl::{pkey::{PKey, Private}, rsa::Rsa, sign::Signer, hash::{MessageDigest, hash, Hasher, DigestBytes}};

use crate::{utils::{create_file_progress_bar, digest_bytes_to_hash, HashType, get_dir_path}, config::get_config};

const PRIVATE_KEY_NAME: &str = "key.private.pkcs8";
const CHUNK_SIZE: usize = 16 * 1024;

fn create_cret() -> Result<PKey<Private>> {
    let path = get_dir_path(PRIVATE_KEY_NAME)?;
    let p = path.as_path();

    let rsa = Rsa::generate(2048).unwrap();
    let enc_private = PKey::from_rsa(rsa).unwrap();

    let mut private_file = File::create(p)?;
    let _ = private_file.write_all(&enc_private.private_key_to_pkcs8()?);

    Ok(enc_private)
}

fn read_cret() -> Result<PKey<Private>> {
    let path = get_dir_path(PRIVATE_KEY_NAME)?;
    let p = path.as_path();

    let file = fs::read(p)?;
    let key = PKey::private_key_from_pkcs8(&file)?;

    Ok(key)
}

fn get_cret() -> Result<PKey<Private>> {
    let path = get_dir_path(PRIVATE_KEY_NAME)?;
    let p = path.as_path();

    let key;

    if !Path::exists(Path::new(p)) {
        key = create_cret()?;
    } else {
        key = read_cret()?;
    }

    Ok(key)
}

fn sign(key: PKey<Private>, data: &[u8]) -> Result<Vec<u8>> {
    let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
    signer.update(data)?;
    
    let signature = signer.sign_to_vec()?;

    Ok(signature)
}

pub fn calc_file_hash(path: &str) -> Result<DigestBytes> {
    let visible = !get_config().invisible;
    let mut file = File::open(path)?;
    let max_size = file.metadata()?.len();

    let mut buffer = [0u8; CHUNK_SIZE];
    let mut hashed = 0;

    let pb = match visible {
        true => Some(create_file_progress_bar("Hashing", max_size)),
        false => None
    };

    let mut hasher = Hasher::new(MessageDigest::sha256())?;

    loop {
        let read_bytes = file.read(&mut buffer)?;

        if read_bytes == 0 {
            break;
        }

        let _ = hasher.update(&buffer[..read_bytes])?;

        hashed += read_bytes;
        if let Some(ref progress) = pb {
            progress.set_position(hashed as u64);
        }
    }

    if let Some(progress) = pb {
        progress.finish_and_clear();
    }

    Ok(hasher.finish()?)
}

pub fn get_public_key() -> Result<String> {
    let key = get_cret()?;
    let public = key.public_key_to_pem()?;

    Ok(String::from_utf8(public).unwrap())
}

pub fn get_public_key_fingerprint() -> Result<String> {
    let key = get_cret()?;
    let x509_public = key.public_key_to_pem()?;

    let hash = hash(MessageDigest::sha256(), &x509_public)?;
    let fingerprint = digest_bytes_to_hash(hash, Some(HashType::Fingerprint));

    Ok(fingerprint)
}

pub fn sign_buffer(data: &[u8]) -> Result<Vec<u8>> {
    Ok(sign(get_cret()?, data)?)
}