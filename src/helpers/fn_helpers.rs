use std::{path::PathBuf, process::Command};

pub fn detect_user() -> String {
    let user_command = Command::new("whoami").output().expect("error command");
    let user = String::from_utf8_lossy(&user_command.stdout);
    user.trim().to_string()
}

pub fn full_path() -> PathBuf {
    let user = detect_user().parse::<String>().expect("error parsing user");
    let mut full_path = PathBuf::new();
    full_path.push("/home");
    full_path.push(user);
    full_path.push(".config");
    full_path.push("zoomark");
    full_path
}

pub fn full_path_w_file() -> PathBuf {
    let user = detect_user().parse::<String>().expect("error parsing user");
    let mut full_path = PathBuf::new();
    full_path.push("/home");
    full_path.push(user);
    full_path.push(".config");
    full_path.push("zoomark");
    full_path.push("command_user");
    full_path.set_extension("json");
    full_path
}
