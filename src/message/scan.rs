use std::fs::read_dir;
use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;

use crate::crypto::rsa::decrypt_with_priv;
use crate::storage::fsio::is_valid_dir;
use crate::storage::paths::{persistent_dir, ram_dir};

pub fn search_messages() -> io::Result<Vec<DirEntry>> {
    search_messages_in(persistent_dir(), ram_dir())
}

pub fn search_messages_in(persistent: &str, ram: &str) -> io::Result<Vec<DirEntry>> {
    if !is_valid_dir(persistent) {
        eprintln!("Couldn't find persistent folder at {persistent}");
        return Ok(Vec::new());
    }
    if !is_valid_dir(ram) {
        eprintln!("Couldn't find RAM folder at {ram}");
        return Ok(Vec::new());
    }
    let ram_paths = read_dir(ram)?;
    let persistent_paths = read_dir(persistent)?;
    let mut all = Vec::new();
    all.extend(ram_paths.filter_map(Result::ok));
    all.extend(persistent_paths.filter_map(Result::ok));
    Ok(all)
}

pub fn get_intended_messages(private_pem: &str) -> io::Result<Vec<(PathBuf, Vec<u8>)>> {
    get_intended_messages_with_pass(private_pem, None)
}

pub fn get_intended_messages_with_pass(
    private_pem: &str,
    pass: Option<&[u8]>,
) -> io::Result<Vec<(PathBuf, Vec<u8>)>> {
    get_intended_messages_in_with_pass(private_pem, pass, persistent_dir(), ram_dir())
}

pub fn get_intended_messages_in_with_pass(
    private_pem: &str,
    pass: Option<&[u8]>,
    persistent: &str,
    ram: &str,
) -> io::Result<Vec<(PathBuf, Vec<u8>)>> {
    let entries = search_messages_in(persistent, ram)?;
    let mut found = Vec::new();
    for entry in entries {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        if let Ok(cipher) = std::fs::read(&p) {
            if let Ok(plain) = decrypt_with_priv(&cipher, private_pem, pass) {
                found.push((p, plain));
            }
        }
    }
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::rsa::encrypt_with_pub;
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;
    use tempfile::tempdir;

    #[test]
    fn search_messages_missing_dirs_yields_empty_ok() -> io::Result<()> {
        let list = search_messages_in("/this/does/not/exist", "/neither/does/this")?;
        assert!(list.is_empty());
        Ok(())
    }

    #[test]
    fn get_intended_messages_returns_only_decryptable() -> io::Result<()> {
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let pk_pem = String::from_utf8(pkey.public_key_to_pem().unwrap()).unwrap();
        let sk_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let d_persistent = tempdir()?;
        let d_ram = tempdir()?;
        let msg1 = b"alpha";
        let msg2 = b"bravo";
        let c1 = encrypt_with_pub(msg1, &pk_pem).unwrap();
        let c2 = encrypt_with_pub(msg2, &pk_pem).unwrap();
        let p1 = d_persistent.path().join("m1.msg");
        let p2 = d_ram.path().join("m2.msg");
        std::fs::write(&p1, &c1)?;
        std::fs::write(&p2, &c2)?;
        let p_bad = d_persistent.path().join("noise.bin");
        std::fs::write(&p_bad, b"\x01\x02\x03\x04\x05")?;
        let list = get_intended_messages_in_with_pass(
            &sk_pem,
            None,
            d_persistent.path().to_str().unwrap(),
            d_ram.path().to_str().unwrap(),
        )?;
        assert_eq!(list.len(), 2);
        let plains: Vec<String> = list
            .iter()
            .map(|(_, v)| String::from_utf8_lossy(v).to_string())
            .collect();
        assert!(plains.contains(&"alpha".to_string()));
        assert!(plains.contains(&"bravo".to_string()));
        Ok(())
    }
}
