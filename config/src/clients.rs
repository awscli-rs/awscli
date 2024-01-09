use aws_sdk_account as account;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_ebs as ebs;
use aws_sdk_ec2 as ec2;
use aws_sdk_eks as eks;

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

    pub fn ec2(&self) -> ec2::Client {
        ec2::Client::new(self.config())
    }

    pub fn eks(&self) -> eks::Client {
        eks::Client::new(self.config())
    }
}
