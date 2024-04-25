use aws_sdk_account as account;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod contact;
mod region;

type AccountResult<T = Box<dyn show::Show>> = Result<T, account::Error>;

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
    async fn execute(self, config: &Config) -> AccountResult {
        match self {
            Self::EnableRegion(enable_region) => enable_region.execute(config).await,
            Self::DisableRegion(disable_region) => disable_region.execute(config).await,
            Self::ListRegions(list_regions) => list_regions.execute(config).await,
            Self::GetRegionOptStatus(status) => status.execute(config).await,
            Self::GetAlternateContact(get_alternate_contact) => {
                get_alternate_contact.execute(config).await
            }
            Self::DeleteAlternateContact(delete_alternate_contact) => {
                delete_alternate_contact.execute(config).await
            }
            Self::PutAlternateContact(put_alternate_contact) => {
                put_alternate_contact.execute(config).await
            }
            Self::GetContactInformation(get_contact_information) => {
                get_contact_information.execute(config).await
            }
            Self::PutContactInformation(put_contact_information) => {
                put_contact_information.execute(config).await
            }
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<account::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
