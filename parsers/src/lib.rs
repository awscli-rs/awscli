use itertools::Itertools;
use serde_json as json;
use thiserror::Error;

pub mod account;
pub mod datetime;
pub mod dynamodb;
pub mod ebs;
pub mod eks;
pub mod sts;
pub mod tag;
