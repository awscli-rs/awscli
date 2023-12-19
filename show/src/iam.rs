use super::*;

impl Show for aws_sdk_iam::types::User {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.user_name()._fmt() } " " { self.user_id()._fmt() }
        ))
    }
}

impl Show for aws_sdk_iam::types::SummaryKeyType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }
}

impl Show for aws_sdk_iam::types::PolicyDetail {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            {self.policy_name()._fmt()} ": " {self.policy_document().map(Rfc3986)._fmt()}
        ))
    }
}

impl Show for aws_sdk_iam::types::AttachedPolicy {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            {self.policy_name()._fmt()} ": " {self.policy_arn()._fmt()}
        ))
    }
}

impl Show for aws_sdk_iam::types::AttachedPermissionsBoundary {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            {self.permissions_boundary_type()._fmt()} "\t" {self.permissions_boundary_arn()._fmt()}
        ))
    }
}

impl Show for aws_sdk_iam::types::PermissionsBoundaryAttachmentType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }
}

impl Show for aws_sdk_iam::types::Tag {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            {self.key()} ": " {self.value()}
        ))
    }
}

impl Show for aws_sdk_iam::types::UserDetail {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "Path:\t\t\t" { self.path()._fmt() } "\n"
            "UserName:\t\t" { self.user_name()._fmt() } "\n"
            "UserId:\t\t\t" { self.user_id()._fmt() } "\n"
            "Arn:\t\t\t" { self.arn()._fmt() } "\n"
            "CreateDate:\t\t" { self.create_date()._fmt() } "\n"
            "UserPolicyList:\t\t" { self.user_policy_list()._fmt() } "\n"
            "GroupList:\t\t" { self.group_list()._fmt_compact() } "\n"
            "AttachedManagedPolicies:\t" { self.attached_managed_policies()._fmt() } "\n"
            "PermissionsBoundary:\t" { self.permissions_boundary()._fmt() } "\n"
            "Tags:\t\t\t" { self.tags()._fmt_compact() } "\n"
        ))
    }
}
