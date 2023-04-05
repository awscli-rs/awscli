alias b := build
build:
    cargo build --workspace
alias t := test
test:
    cargo test --workspace
alias c:=clippy
clippy:
    cargo clippy --workspace --all-targets
clean:
    cargo clean
update:
    cargo update
deprecated:
    cargo clippy --features clap/deprecated
fresh: clean update clippy test build
