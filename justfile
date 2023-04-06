alias b := build
build:
    cargo build --workspace
alias t := test
test:
    cargo test --workspace
alias c:=clippy
clippy:
    cargo clippy --workspace --all-targets -- \
        --warn clippy::use_self \
        --warn clippy::map_flatten \
        --warn clippy::map_unwrap_or \
        --warn deprecated_in_future \
        --warn future_incompatible \
        --warn noop_method_call \
        --warn unreachable_pub \
        --warn missing_debug_implementations \
        --warn rust_2018_compatibility \
        --warn rust_2021_compatibility \
        --warn rust_2018_idioms \
        --warn trivial_casts \
        --warn unused \
        --deny warnings

clean:
    cargo clean
update:
    cargo update
deprecated:
    cargo clippy --features clap/deprecated
fresh: clean update clippy test build
