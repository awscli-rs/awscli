use super::*;

impl Show for aws_sdk_eks::types::Cluster {
    fn text(&self) -> String {
        let name = self.name().unwrap_or_default();
        let version = self.version().unwrap_or_default();

        fmtools::format!(
            {name} " " {version}
        )
    }
}

impl Show for aws_sdk_eks::types::UpdateStatus {
    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_eks::types::UpdateType {
    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_eks::types::UpdateParam {
    fn text(&self) -> String {
        fmtools::format!("TYPE\t" {self.r#type().text()})
    }
}

impl Show for aws_sdk_eks::types::UpdateParamType {
    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_eks::types::Update {
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
