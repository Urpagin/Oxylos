use colored::Colorize;
use std::io;
use std::path::Path;

pub fn ensure_storage_or_init() -> io::Result<()> {
    let path = Path::new("storage");
    if !path.exists() {
        init()?;
        std::process::exit(0);
    }
    Ok(())
}

fn create_persistent_storage() -> io::Result<()> {
    std::fs::create_dir_all("storage")?;
    std::fs::create_dir_all("storage/keys")?;
    std::fs::create_dir_all("storage/index")?;
    std::fs::create_dir_all("storage/messages")?;
    Ok(())
}

pub fn init() -> io::Result<()> {
    let color_text = "Olyxos -h";
    println!(
        "Thank you for using Olyxos. This is the initialization of the program; it creates 4 directories. The first one is storage: this is where the program stores everything. If you already have a pair of cryptographic keys and you want to use them, please put them in storage/keys/ under the names 'private.pem' and 'public.pem'. You can see the arguments to use by running {}",
        color_text.red()
    );
    create_persistent_storage()
}

pub fn ensure_os_dropboxes() -> io::Result<()> {
    use std::fs::create_dir_all;
    let p = crate::storage::paths::persistent_dir();
    let r = crate::storage::paths::ram_dir();
    let _ = create_dir_all(p);
    let _ = create_dir_all(r);
    Ok(())
}
