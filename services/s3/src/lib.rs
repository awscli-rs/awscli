use async_trait::async_trait;
use aws_sdk_s3 as s3;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

use self::client::S3Client;
use self::ext::bucket_name;
use self::ext::maybe_bucket_name;

mod bucket;
mod client;
mod ext;

type S3Result<T = Box<dyn show::Show>> = std::result::Result<T, s3::Error>;

const S3_PREFIX: &str = "s3://";

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: s3::Client) -> S3Result;
}

/// High-level S3 commands
#[derive(Debug, Subcommand)]
pub enum S3 {
    Rb(bucket::Remove),
    Ls(bucket::List),
}

impl S3 {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::Rb(rb) => Box::new(rb),
            Self::Ls(ls) => Box::new(ls),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<s3::Error>> {
        let client = s3::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
