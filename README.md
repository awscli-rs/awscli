# AWS CLI alternative written in Rust

## Code layout overview

1. Each service support lives in its own crate under services/
2. Text output is driven by implementing show::Show trait via config::Config object.
