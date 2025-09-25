#[cfg(target_os = "windows")]
pub fn persistent_dir() -> &'static str {
    r"C:\ProgramData\Oxylos\"
}

#[cfg(target_os = "windows")]
pub fn ram_dir() -> &'static str {
    r"Global\Oxylos"
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn persistent_dir() -> &'static str {
    "/tmp/Oxylos/"
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn ram_dir() -> &'static str {
    "/dev/shm/Oxylos/"
}

#[cfg(target_os = "macos")]
pub fn persistent_dir() -> &'static str {
    "/tmp/Oxylos/"
}

#[cfg(target_os = "macos")]
pub fn ram_dir() -> &'static str {
    "/tmp/Oxylos.shm/"
}
