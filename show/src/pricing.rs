use super::*;

impl Show for aws_sdk_pricing::types::Service {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("SERVICES", self.service_code()) } "\n"
            { prefixed_items("ATTRIBUTENAMES", self.attribute_names()) }

        ))
    }

    fn text(&self) -> String {
        let service_code = self.service_code().unwrap_or_default();
        let attribute_names = self.attribute_names().unwrap_or_default();

        fmtools::format!(
            "SERVICES\t" {service_code} "\n"
            for attr in attribute_names {
                "ATTRIBUTENAMES\t"{attr} "\n"
            }
        )
    }
}

impl Show for aws_sdk_pricing::types::AttributeValue {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!({ self.value()._fmt() }))
    }

    fn text(&self) -> String {
        self.value().unwrap_or_default().to_string()
    }
}

impl Show for aws_sdk_pricing::types::PriceList {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "PRICELISTS\t"
            { self.currency_code()._fmt() } "\t"
            { self.price_list_arn()._fmt() } "\t"
            { self.region_code()._fmt() } "\n"
            { prefixed_items("FILEFORMATS", self.file_formats()) }
        ))
    }

    fn text(&self) -> String {
        let arn = self.price_list_arn().unwrap_or_default();
        let currency = self.currency_code().unwrap_or_default();
        let region = self.region_code().unwrap_or_default();
        let formats = self.file_formats().unwrap_or_default();
        fmtools::format!(
            "PRICELISTS\t" {currency} "\t" {arn} "\t"  {region} "\n"
            for format in formats {
                "FILEFORMATS\t" {format} "\n"
            }
        )
    }
}
