use anyhow::{Context, Result};
use chrono::Utc;
use std::io;
use std::path::Path;
use uuid::Uuid;

use crate::cli::Args;
use crate::config::DEFAULT_OUT_MSG;
use crate::crypto::rsa::encrypt_with_pub;
use crate::storage::fsio::write_all;

pub fn run(args: &Args) -> Result<()> {
    let _t = Utc::now();
    let uuid = Uuid::new_v4();
    let out = format!("{DEFAULT_OUT_MSG}{uuid}");
    let out_path = args.path.as_deref().unwrap_or(Path::new(&out));

    let mut author = String::new();
    println!("enter your name/pseudonyme below:");
    io::stdin().read_line(&mut author).context("stdin author")?;

    let mut content = String::new();
    println!("enter your text below:");
    io::stdin()
        .read_line(&mut content)
        .context("stdin content")?;

    let pk_pem = std::fs::read_to_string(&args.public_key).context("read public key failed")?;
    let payload = format!("Author: {author}\n{content}");
    let cipher_text = encrypt_with_pub(payload.trim_end().as_bytes(), &pk_pem)?;
    write_all(out_path, &cipher_text)?;
    Ok(())
}
