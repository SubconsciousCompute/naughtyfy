cargo fmt &&
    cargo check &&
    cargo test &&
    cargo clippy &&
    cargo build --release &&
    cargo clean
