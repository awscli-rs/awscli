use aws_sdk_account::types::ContactInformation;

use super::*;

const ADDRESS_LINE1: &str = "AddressLine1";
const ADDRESS_LINE2: &str = "AddressLine2";
const ADDRESS_LINE3: &str = "AddressLine3";
const CITY: &str = "City";
const COMPANY_NAME: &str = "CompanyName";
const COUNTRY_CODE: &str = "CountryCode";
const DISTRICT_OR_COUNTY: &str = "DistrictOrCounty";
const FULL_NAME: &str = "FullName";
const PHONE_NUMBER: &str = "PhoneNumber";
const POSTAL_CODE: &str = "PostalCode";
const STATE_OR_REGION: &str = "StateOrRegion";
const WEBSITE_URL: &str = "WebsiteUrl";

// AddressLine1=string,AddressLine2=string,AddressLine3=string,City=string,
// CompanyName=string,CountryCode=string,DistrictOrCounty=string,FullName=string,
// PhoneNumber=string,PostalCode=string,StateOrRegion=string,WebsiteUrl=string

pub fn contact_information(text: &str) -> Result<ContactInformation, InvalidConfigInformation> {
    json::from_str::<json::Value>(text).map_or_else(|_| from_text(text), from_json)
}

fn from_json(value: json::Value) -> Result<ContactInformation, InvalidConfigInformation> {
    let address_line1 = get_string_item(&value, ADDRESS_LINE1);
    let address_line2 = get_string_item(&value, ADDRESS_LINE2);
    let address_line3 = get_string_item(&value, ADDRESS_LINE3);
    let city = get_string_item(&value, CITY);
    let company_name = get_string_item(&value, COMPANY_NAME);
    let country_code = get_string_item(&value, COUNTRY_CODE);
    let district_or_county = get_string_item(&value, DISTRICT_OR_COUNTY);
    let full_name = get_string_item(&value, FULL_NAME);
    let phone_number = get_string_item(&value, PHONE_NUMBER);
    let postal_code = get_string_item(&value, POSTAL_CODE);
    let state_or_region = get_string_item(&value, STATE_OR_REGION);
    let website_url = get_string_item(&value, WEBSITE_URL);

    let contact_info = ContactInformation::builder()
        .set_address_line1(address_line1)
        .set_address_line2(address_line2)
        .set_address_line3(address_line3)
        .set_city(city)
        .set_company_name(company_name)
        .set_country_code(country_code)
        .set_district_or_county(district_or_county)
        .set_full_name(full_name)
        .set_phone_number(phone_number)
        .set_postal_code(postal_code)
        .set_state_or_region(state_or_region)
        .set_website_url(website_url)
        .build();

    Ok(contact_info)
}

fn get_string_item(value: &json::Value, item: &str) -> Option<String> {
    value[item].as_str().map(ToString::to_string)
}

fn from_text(text: &str) -> Result<ContactInformation, InvalidConfigInformation> {
    let value = text
        .split(',')
        .filter_map(|item| item.split_once('='))
        .collect::<json::Value>();
    from_json(value)
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum InvalidConfigInformation {
    #[error("Unknown Attribute: '{0}'")]
    UnknownAttribute(String),
}

impl InvalidConfigInformation {
    fn _unknown(text: &str) -> Self {
        Self::UnknownAttribute(text.to_string())
    }
}
