alias b := build
build:
    cargo build --workspace -Zlints
alias t := test
test:
    cargo test --workspace -Zlints
raws *ARGS:
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
deps:
    cargo update && git add Cargo.lock && git ci -m "Update deps"
