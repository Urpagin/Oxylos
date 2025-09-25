use anyhow::{Context, Result};
use std::io::{self, Write};

use crate::cli::Args;
use crate::message::scan::get_intended_messages_with_pass;

pub fn run(args: &Args) -> Result<()> {
    let private_pem = std::fs::read_to_string(&args.private_key).context("read private key failed")?;
    let pass = args.passphrase.as_deref().map(str::as_bytes);
    let items = get_intended_messages_with_pass(&private_pem, pass)?;
    for (path, plain) in items {
        println!("{}\n---", path.display());
        io::stdout().write_all(&plain)?;
        println!();
    }
    Ok(())
}

