use filetime::{set_file_times, FileTime};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

fn neutralize_metadata(path: &Path) -> io::Result<()> {
    let perms = fs::Permissions::from_mode(0o600);
    let _ = fs::set_permissions(path, perms);
    let zero = FileTime::from_unix_time(0, 0);
    let _ = set_file_times(path, zero, zero);
    Ok(())
}

pub fn write_all(path: &Path, content: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
    f.write_all(content)?;
    f.sync_all()?;
    neutralize_metadata(path)
}

pub fn list_files(directory_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        paths.push(entry.path());
    }
    Ok(paths)
}

pub fn is_valid_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn copy_msg(source: &str, destination: &str) -> io::Result<()> {
    let mut buf = Vec::new();
    File::open(source)?.read_to_end(&mut buf)?;
    write_all(Path::new(destination), &buf)
}
