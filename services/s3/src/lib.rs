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

type S3Result<T = Box<dyn show::Show>> = Result<T, s3::Error>;

const S3_PREFIX: &str = "s3://";

/// High-level S3 commands
#[derive(Debug, Subcommand)]
pub enum S3 {
    Mb(bucket::Make),
    Rb(bucket::Remove),
    Ls(bucket::List),
}

impl S3 {
    async fn execute(self, config: &Config) -> S3Result {
        match self {
            Self::Mb(mb) => mb.execute(config).await,
            Self::Rb(rb) => rb.execute(config).await,
            Self::Ls(ls) => ls.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<s3::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
