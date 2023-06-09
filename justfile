alias b := build
build:
    cargo build --workspace -Zlints
alias t := test
test:
    cargo test --workspace -Zlints
run *ARGS:
    cargo run -Zlints {{ARGS}}
alias c:=clippy
clippy:
    cargo clippy --workspace --all-targets -Zlints
clean:
    cargo clean
update:
    cargo update
deprecated:
    cargo clippy --features clap/deprecated
fresh: clean update clippy test build
