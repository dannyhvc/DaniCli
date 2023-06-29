pub fn windows_to_unix_path(windows_path: &str) -> String {
    let mut unix_path = windows_path.replace("\\", "/");
    if unix_path.starts_with("C:") {
        unix_path = unix_path.replacen("C:", "/mnt/c", 1);
    }
    unix_path
}

pub fn unix_to_windows_path(unix_path: &str) -> String {
    let mut windows_path = unix_path.replace("/", "\\");
    if windows_path.starts_with("C:") {
        windows_path = windows_path.replacen("C:", "/mnt/c", 1);
    }
    windows_path
}
