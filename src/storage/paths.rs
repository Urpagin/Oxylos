/// Exits at compile-time or panics if no suitable OS found.
/// (suitable = {windows,macos,unix})
fn no_os_found() -> ! {
    // Maybe use a temporary local directory, here?

    // This fails at compile-time.
    #[cfg(not(any(target_os = "macos", unix, windows)))]
    compile_error!("OS not detected!");

    // If somehow, it doesn't, we panic.
    panic!("OS not detected!");
}

/// Returns an OS specific persistent directory to store app data.
///
/// Also, maybe consider returning a &Path instead of a &str.
pub fn persistent_dir() -> &'static str {
    // MacOS is Unix. And the path is no different.
    if cfg!(unix) {
        "/tmp/Oxylos/"
    } else if cfg!(windows) {
        r"C:\ProgramData\Oxylos\"
    } else {
        no_os_found()
    }
}

/// Returns an OS specific temporary directory to store app data.
///
/// Stores directly on the shm device on unix, on the RAM.
pub fn ram_dir() -> &'static str {
    // MacOS first because MacOS is Unix.
    if cfg!(target_os = "macos") {
        "/tmp/Oxylos.shm/"
    } else if cfg!(unix) {
        "/dev/shm/Oxylos/"
    } else if cfg!(windows) {
        r"Global\Oxylos"
    } else {
        no_os_found()
    }
}
