use super::*;

#[derive(Debug, Args)]
pub struct DeleteUser {
    /// The name of the user to create.
    ///
    /// IAM user, group, role, and policy names must be unique within the
    /// account. Names are not distinguished by case. For example, you
    /// cannot create resources named both "MyResource" and "myresource".
    #[arg(long)]
    user_name: String,
}

impl DeleteUser {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        config
            .iam()
            .delete_user()
            .user_name(self.user_name)
            .send()
            .await?;
        Ok(Box::new(()))
    }
}
