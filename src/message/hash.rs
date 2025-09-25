use openssl::sha::Sha256;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn sha256_file(path: &Path) -> io::Result<[u8; 32]> {
    let file = File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finish())
}

