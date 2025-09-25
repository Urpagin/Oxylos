use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = oxylos::cli::Args::parse();
    oxylos::storage::init::ensure_storage_or_init()?;
    oxylos::storage::init::ensure_os_dropboxes()?;
    let _ = oxylos::message::sync::auto_sync()?;
    oxylos::commands::dispatch(args)
}
