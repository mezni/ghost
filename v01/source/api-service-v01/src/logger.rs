use env_logger::{Builder, Target};
use log::LevelFilter;
use std::env;
use std::io::Write;

pub fn init_logger() {
    // Default to info level if RUST_LOG is not set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .parse_default_env()
        .init();
}
