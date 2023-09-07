use super::*;

impl Show for aws_sdk_eks::types::Cluster {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("NAME", self.name()) } "\n"
            { prefixed_item("VERSION", self.version()) }
        ))
    }

    fn text(&self) -> String {
        let name = self.name().unwrap_or_default();
        let version = self.version().unwrap_or_default();

        fmtools::format!(
            {name} " " {version}
        )
    }
}

impl Show for aws_sdk_eks::types::UpdateStatus {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_eks::types::UpdateType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_eks::types::UpdateParam {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        prefixed_item("TYPE", self.r#type())
    }

    fn text(&self) -> String {
        fmtools::format!("TYPE\t" {self.r#type().text()})
    }
}

impl Show for aws_sdk_eks::types::UpdateParamType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_eks::types::Update {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("ID", self.id()) } "\n"
            { prefixed_item("STATUS", self.status()) } "\n"
            { prefixed_item("TYPE", self.r#type()) } "\n"
            { prefixed_item("PARAMS", self.params()) }

        ))
    }

    fn text(&self) -> String {
        fmtools::format!(
            "id " {self.id().text()} "\n"
            "status " {self.status.text()} "\n"
            "type " {self.r#type.text()} "\n"
            "params " {self.params().text()}
        )
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

    fn text(&self) -> String {
        let name = self.addon_name().unwrap_or_default();
        let status = self
            .status()
            .map(|status| status.as_str())
            .unwrap_or_default();
        let version = self.addon_version().unwrap_or_default();
        let arn = self.addon_arn().unwrap_or_default();

        fmtools::format!(
            "name " {name} "\n"
            "status " {status} "\n"
            "version " {version} "\n"
            "arn " {arn}
        )
    }
}

impl Show for aws_sdk_eks::types::AddonStatus {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        todo!()
    }
}
