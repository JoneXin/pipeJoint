use dotenv::dotenv;
use std::env::{self, VarError};

#[derive(Debug)]
pub struct LogsStruct<'a> {
    pub show_terminal_color: bool,
    pub persistent_logs: bool,
    pub logs_dir: &'a str,
}

pub fn get_logs_conf<'a>() -> LogsStruct<'a> {
    dotenv().ok();

    let rust_env: Result<String, VarError> = env::var("RUST_ENV");
    let mut logs_config = LogsStruct {
        show_terminal_color: false,
        persistent_logs: true,
        logs_dir: "./logs",
    };

    match rust_env {
        Ok(r_env) => {
            if r_env == "development" {
                logs_config = LogsStruct {
                    show_terminal_color: true,
                    persistent_logs: false,
                    logs_dir: "../logs",
                }
            }
        }
        _ => {}
    }

    // println!("logs config:{:#?}", logs_config);
    logs_config
}
