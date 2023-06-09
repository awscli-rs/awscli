use super::*;

impl Show for aws_sdk_account::types::Region {
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

impl Show for aws_sdk_account::operation::get_region_opt_status::GetRegionOptStatusOutput {
    fn text(&self) -> String {
        let name = self.region_name().unwrap_or_default();
        let opt_status = self
            .region_opt_status()
            .map(|status| status.as_str())
            .unwrap_or_default();
        fmtools::format!(
            {name} " " {opt_status}
        )
    }
}
