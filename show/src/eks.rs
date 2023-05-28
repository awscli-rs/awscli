use super::*;

impl Show for aws_sdk_eks::types::Cluster {
    fn text(&self) -> String {
        let name = self.name().unwrap_or_default();
        let version = self.version().unwrap_or_default();
        fmtools::format!(
            {name} " " {version}
        )
    }

    fn debug(&self) -> String {
        format!("{self:?}")
    }
}

impl Show for aws_sdk_eks::types::Update {
    fn text(&self) -> String {
        let id = self.id().unwrap_or_default();
        let status = self
            .status()
            .map(|status| status.as_str())
            .unwrap_or_default();
        let r#type = self
            .r#type()
            .map(|r#type| r#type.as_str())
            .unwrap_or_default();
        let params = self.params().unwrap_or_default();
        fmtools::format!(
            "id " {id} "\n"
            "status " {status} "\n"
            "type " {r#type} "\n"
            "params " {params:?}
        )
    }

    fn debug(&self) -> String {
        format!("{self:?}")
    }
}
