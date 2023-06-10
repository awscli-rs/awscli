use super::*;

pub(crate) use alternate::DeleteAlternateContact;
pub(crate) use alternate::GetAlternateContact;
pub(crate) use alternate::PutAlternateContact;
pub(crate) use information::GetContactInformation;
pub(crate) use information::PutContactInformation;

mod alternate;
mod information;
