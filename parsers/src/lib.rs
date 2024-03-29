use aws_smithy_types::error::operation::BuildError;

use itertools::Itertools;
use serde_json as json;
use thiserror::Error;

pub mod account;
pub mod datetime;
pub mod dynamodb;
pub mod ec2;
pub mod eks;
pub mod sts;
pub mod tag;
