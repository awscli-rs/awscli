use super::*;

impl Show for aws_sdk_eks::types::Cluster {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("NAME", self.name()) } "\n"
            { prefixed_item("VERSION", self.version()) }
        ))
    }
}

impl Show for aws_sdk_eks::types::UpdateStatus {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for aws_sdk_eks::types::UpdateType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for aws_sdk_eks::types::UpdateParam {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        prefixed_item("TYPE", self.r#type())
    }
}

impl Show for aws_sdk_eks::types::UpdateParamType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for aws_sdk_eks::types::Update {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("ID", self.id()) } "\n"
            { prefixed_item("STATUS", self.status()) } "\n"
            { prefixed_item("TYPE", self.r#type()) } "\n"
            { prefixed_items("PARAMS", self.params()) }

        ))
    }
}

impl Show for aws_sdk_eks::types::Addon {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("NAME", self.addon_name()) } "\n"
            { prefixed_item("STATUS", self.status()) } "\n"
            { prefixed_item("VERSION", self.addon_version()) } "\n"
            { prefixed_item("ARN", self.addon_arn()) }
        ))
    }
}

impl Show for aws_sdk_eks::types::AddonStatus {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}
