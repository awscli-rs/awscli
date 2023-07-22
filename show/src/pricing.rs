use super::*;

impl Show for aws_sdk_pricing::types::Service {
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
    fn text(&self) -> String {
        self.value().unwrap_or_default().to_string()
    }
}

impl Show for aws_sdk_pricing::types::PriceList {
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

impl Show for aws_sdk_pricing::operation::get_price_list_file_url::GetPriceListFileUrlOutput {
    fn text(&self) -> String {
        let url = self.url().unwrap_or_default();
        fmtools::format!({ url })
    }
}
