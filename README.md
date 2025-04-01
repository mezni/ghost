

[workspace]
members = [
    "core",
    "loader-service",
]


cargo new --lib core
cargo new --bin loader-service


cargo run --bin loader-service --target-dir bin

cargo run -p loader-service
