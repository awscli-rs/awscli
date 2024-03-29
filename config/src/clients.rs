use aws_sdk_account as account;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_ebs as ebs;
use aws_sdk_ec2 as ec2;
use aws_sdk_eks as eks;
use aws_sdk_iam as iam;
use aws_sdk_pricing as pricing;
use aws_sdk_s3 as s3;
use aws_sdk_sso as sso;
use aws_sdk_sts as sts;

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

    pub fn iam(&self) -> iam::Client {
        iam::Client::new(self.config())
    }

    pub fn pricing(&self) -> pricing::Client {
        pricing::Client::new(self.config())
    }

    pub fn s3(&self) -> s3::Client {
        s3::Client::new(self.config())
    }

    pub fn sso(&self) -> sso::Client {
        sso::Client::new(self.config())
    }

    pub fn sts(&self) -> sts::Client {
        sts::Client::new(self.config())
    }
}
