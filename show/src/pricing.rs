use super::*;

impl Show for aws_sdk_pricing::types::Service {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("SERVICES", self.service_code()) } "\n"
            { prefixed_items("ATTRIBUTENAMES", self.attribute_names()) }

        ))
    }
}

impl Show for aws_sdk_pricing::types::AttributeValue {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!({ self.value()._fmt() }))
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
}
