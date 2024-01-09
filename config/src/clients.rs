use aws_sdk_account as account;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_ebs as ebs;

use super::*;

impl Config {
    pub fn account(&self) -> account::Client {
        account::Client::new(self.config())
    }

    pub fn dynamodb(&self) -> dynamodb::Client {
        dynamodb::Client::new(self.config())
    }

    pub fn ebs(&self) -> ebs::Client {
        ebs::Client::new(self.config())
    }
}
