# AWS CLI alternative written in Rust

## Code layout overview

1. Each service support lives in its own crate under services/
2. Text output is driven by implementing show::Show trait via config::Config object.

## Adding new service

1. workspace Cargo.toml - add respective aws-sdk-xxx crate as new dependency
2. cargo new --lib services/xxx
3. workspace Cargo.toml - add new crate to members
4. services/xxx/Cargo.toml
  1. add "description = RAWS xxx component"
  2. add dependencies
  3. add lints
5. in error crate: impl AwsError for xxx::Error
6. in show crate: impl Show for xxx::types::Xxx
