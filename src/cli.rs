use clap::Parser;
use std::path::PathBuf;

use crate::config::{DEFAULT_PRIVKEY, DEFAULT_PUBKEY};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(short = 'e', long = "encrypt", conflicts_with_all = ["decrypt", "inbox"])]
    pub encrypt: bool,

    #[arg(short = 'd', long = "decrypt", conflicts_with_all = ["encrypt", "inbox"])]
    pub decrypt: bool,

    #[arg(long = "inbox", conflicts_with_all = ["encrypt", "decrypt"])]
    pub inbox: bool,

    #[arg(short = 'p', long = "path")]
    pub path: Option<PathBuf>,

    #[arg(short = 'g', long = "generate-new-keys", conflicts_with_all = ["encrypt", "decrypt", "inbox"])]
    pub generate_new_keys: bool,

    #[arg(long = "public-key", default_value = DEFAULT_PUBKEY)]
    pub public_key: PathBuf,

    #[arg(long = "private-key", default_value = DEFAULT_PRIVKEY)]
    pub private_key: PathBuf,

    #[arg(long = "passphrase")]
    pub passphrase: Option<String>,
}

