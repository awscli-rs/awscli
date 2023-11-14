use super::*;

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user to create.
    #[arg(long)]
    user_name: String,
}

#[async_trait]
impl Execute for CreateUser {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        let user = config
            .client()
            .create_user()
            .user_name(self.user_name)
            .send()
            .await?
            .user;
        Ok(Box::new(user))
    }
}
