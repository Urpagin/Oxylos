use anyhow::{bail, Context, Result};
use std::io::{self, Write};

use crate::cli::Args;
use crate::crypto::rsa::decrypt_with_priv;

pub fn run(args: &Args) -> Result<()> {
    let private_pem = std::fs::read_to_string(&args.private_key).context("read private key failed")?;
    let pass = args.passphrase.as_ref().map(|s| s.as_bytes());
    let in_path = match args.path.as_ref() {
        Some(p) => p,
        None => bail!("--path is required for --decrypt"),
    };
    let ciphertext = std::fs::read(in_path).context("read ciphertext failed")?;
    let plain = decrypt_with_priv(&ciphertext, &private_pem, pass)?;
    io::stdout().write_all(&plain)?;
    Ok(())
}

