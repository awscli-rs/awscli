use super::*;

impl Show for aws_sdk_account::types::Region {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "REGIONS\t"
            { self.region_name()._fmt() } "\t"
            { self.region_opt_status()._fmt() }
        ))
    }
}

impl Show for aws_sdk_account::operation::get_region_opt_status::GetRegionOptStatusOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.region_name()._fmt() } "\t"
            { self.region_opt_status()._fmt() }
        ))
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
}

impl Show for aws_sdk_account::types::RegionOptStatus {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for aws_sdk_account::types::AlternateContactType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}
