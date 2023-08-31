use super::*;

pub(crate) use role::AssumeRole;
pub(crate) use saml::AssumeRoleWithSaml;
pub(crate) use web::AssumeRoleWithWebIdentity;

mod role;
mod saml;
mod web;
