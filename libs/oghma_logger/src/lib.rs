use log::debug;
use std::env;

fn default_rust_log() -> String {
    let ignore_list = [
        "tracing::span=warn",
        "serenity::gateway::ws_client_ext=warn",
    ];

    let mode = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "INFO"
    };

    format!("{},{}", mode, ignore_list.join(","))
}

pub fn init() {
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or_else(|_| default_rust_log()),
    );
    env_logger::init();
    debug!("Initialized Logger <3");
}
