use super::*;

impl Show for aws_sdk_ec2::types::Vpc {
    fn text(&self) -> String {
        let vpc_id = self.vpc_id().unwrap_or_default();
        let state = self.state().map(|state| state.as_str()).unwrap_or_default();

        fmtools::format!(
            {vpc_id} " " {state}
        )
    }
}
