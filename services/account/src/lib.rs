use async_trait::async_trait;
use aws_sdk_account as account;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod contact;
mod region;

type AccountResult<T = Box<dyn show::Show>> = std::result::Result<T, aws_sdk_account::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, config: &Config) -> AccountResult;
}

trait Client {
    fn client(config: &Config) -> account::Client {
        account::Client::new(config.config())
    }
}

impl<T> Client for T where T: Execute {}

/// Operations for Amazon Web Services Account Management
#[derive(Debug, Subcommand)]
pub enum Account {
    EnableRegion(region::EnableRegion),
    DisableRegion(region::DisableRegion),
    ListRegions(region::ListRegions),
    GetRegionOptStatus(region::GetRegionOptStatus),
    GetAlternateContact(contact::GetAlternateContact),
    DeleteAlternateContact(contact::DeleteAlternateContact),
    PutAlternateContact(contact::PutAlternateContact),
    GetContactInformation(contact::GetContactInformation),
    PutContactInformation(contact::PutContactInformation),
}

impl Account {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::EnableRegion(enable_region) => Box::new(enable_region),
            Self::DisableRegion(disable_region) => Box::new(disable_region),
            Self::ListRegions(list_regions) => Box::new(list_regions),
            Self::GetRegionOptStatus(status) => Box::new(status),
            Self::GetAlternateContact(get_alternate_contact) => Box::new(get_alternate_contact),
            Self::DeleteAlternateContact(delete_alternate_contact) => {
                Box::new(delete_alternate_contact)
            }
            Self::PutAlternateContact(put_alternate_contact) => Box::new(put_alternate_contact),
            Self::GetContactInformation(get_contact_information) => {
                Box::new(get_contact_information)
            }
            Self::PutContactInformation(put_contact_information) => {
                Box::new(put_contact_information)
            }
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<aws_sdk_account::Error>> {
        self.boxed()
            .execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
