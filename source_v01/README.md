cargo new --lib logger

[workspace]
members = [
    "core",
    "loader-service", "logger",
]

cargo build --bin loader-service --target-dir bin