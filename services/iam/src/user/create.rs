use super::*;

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user to create.
    #[arg(long)]
    user_name: String,
}

impl CreateUser {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        let user = config
            .iam()
            .create_user()
            .user_name(self.user_name)
            .send()
            .await?
            .user;
        Ok(Box::new(user))
    }
}
