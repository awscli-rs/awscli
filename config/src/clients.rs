use aws_sdk_account as account;
use aws_sdk_dynamodb as dynamodb;

use super::*;

impl Config {
    pub fn account(&self) -> account::Client {
        account::Client::new(self.config())
    }

    pub fn dynamodb(&self) -> dynamodb::Client {
        dynamodb::Client::new(self.config())
    }
}
