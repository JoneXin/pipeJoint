use std::{env, error::Error, path::Path};

pub fn join_cwd_path(path: &str) -> String {
    let config_file = Path::new(&env::current_dir().unwrap()).join(path);
    let binding: std::ffi::OsString = config_file.into_os_string();
    let cwd: Option<&str> = binding.to_str();

    match cwd {
        Some(r_path) => r_path.to_string(),
        None => "cwd path is none".into(),
    }
}
