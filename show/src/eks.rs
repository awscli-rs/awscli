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
