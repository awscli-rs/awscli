use super::*;

impl Show for aws_sdk_account::types::Region {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "REGIONS\t"
            { self.region_name()._fmt() } "\t"
            { self.region_opt_status()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let name = self.region_name().unwrap_or_default();
        let opt_status = self
            .region_opt_status()
            .map(|status| status.as_str())
            .unwrap_or_default();
        fmtools::format!(
            "REGIONS\t" {name} "\t" {opt_status}
        )
    }
}

impl Show for aws_sdk_account::operation::get_region_opt_status::GetRegionOptStatusOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.region_name()._fmt() } "\t"
            { self.region_opt_status()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let name = self.region_name().unwrap_or_default();
        let opt_status = self
            .region_opt_status()
            .map(|status| status.as_str())
            .unwrap_or_default();
        fmtools::format!(
            {name} "\t" {opt_status}
        )
    }
}

impl Show for aws_sdk_account::types::AlternateContact {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "NAME  " { self.name()._fmt() } "\n"
            "TITLE " { self.title()._fmt() } "\n"
            "EMAIL " { self.email_address()._fmt() } "\n"
            "PHONE " { self.phone_number()._fmt() } "\n"
            "TYPE  " { self.alternate_contact_type()._fmt( )}
        ))
    }

    fn text(&self) -> String {
        let name = self.name().unwrap_or_default();
        let title = self.title().unwrap_or_default();
        let email = self.email_address().unwrap_or_default();
        let phone = self.phone_number().unwrap_or_default();
        let r#type = self
            .alternate_contact_type()
            .map(|r#type| r#type.as_str())
            .unwrap_or_default();

        fmtools::format!(
            "NAME  " {name} "\n"
            "TITLE " {title} "\n"
            "EMAIL " {email} "\n"
            "PHONE " {phone} "\n"
            "TYPE  " {r#type}
        )
    }
}

impl Show for aws_sdk_account::types::ContactInformation {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "AddressLine1     " { self.address_line1()._fmt() } "\n"
            "AddressLine2     " { self.address_line2()._fmt() } "\n"
            "AddressLine3     " { self.address_line3()._fmt() } "\n"
            "City             " { self.city()._fmt() }  "\n"
            "CompanyName      " { self.company_name()._fmt() } "\n"
            "CountryCode      " { self.country_code()._fmt() } "\n"
            "DistrictOrCounty " { self.district_or_county()._fmt() } "\n"
            "FullName         " { self.full_name()._fmt() } "\n"
            "PhoneNumner      " { self.phone_number()._fmt() } "\n"
            "PostalCode       " { self.postal_code()._fmt() } "\n"
            "StateOrRegion    " { self.state_or_region()._fmt() } "\n"
            "WebsiteUrl       " { self.website_url()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let address_line1 = self.address_line1().unwrap_or_default();
        let address_line2 = self.address_line2().unwrap_or_default();
        let address_line3 = self.address_line3().unwrap_or_default();
        let city = self.city().unwrap_or_default();
        let company_name = self.company_name().unwrap_or_default();
        let country_code = self.country_code().unwrap_or_default();
        let district_or_county = self.district_or_county().unwrap_or_default();
        let full_name = self.full_name().unwrap_or_default();
        let phone_number = self.phone_number().unwrap_or_default();
        let postal_code = self.postal_code().unwrap_or_default();
        let state_or_region = self.state_or_region().unwrap_or_default();
        let website_url = self.website_url().unwrap_or_default();

        fmtools::format!(
            "AddressLine1     " {address_line1} "\n"
            "AddressLine2     " {address_line2} "\n"
            "AddressLine3     " {address_line3} "\n"
            "City             " {city} "\n"
            "CompanyName      " {company_name} "\n"
            "CountryCode      " {country_code} "\n"
            "DistrictOrCounty " {district_or_county} "\n"
            "FullName         " {full_name} "\n"
            "PhoneNumner      " {phone_number} "\n"
            "PostalCode       " {postal_code} "\n"
            "StateOrRegion    " {state_or_region} "\n"
            "WebsiteUrl       " {website_url}
        )
    }
}

impl Show for aws_sdk_account::types::RegionOptStatus {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        todo!()
    }
}

impl Show for aws_sdk_account::types::AlternateContactType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        todo!()
    }
}
