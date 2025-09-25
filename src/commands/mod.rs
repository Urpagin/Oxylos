use anyhow::Result;

use crate::cli::Args;

pub mod decrypt;
pub mod encrypt;
pub mod inbox;

pub fn dispatch(args: Args) -> Result<()> {
    if args.generate_new_keys {
        crate::crypto::rsa::generate_keys(
            crate::config::DEFAULT_PRIVKEY,
            crate::config::DEFAULT_PUBKEY,
        )?;
        return Ok(());
    }
    if args.encrypt {
        return encrypt::run(&args);
    }
    if args.decrypt {
        return decrypt::run(&args);
    }
    if args.inbox {
        return inbox::run(&args);
    }
    let _ = crate::message::scan::search_messages()?;
    Ok(())
}
