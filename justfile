default: run

run:
    git pull --rebase --autostash
    git submodule update --init --remote
    RUST_LOG=debug cargo run

check:
    cargo clippy -- -D warnings
    cargo test
