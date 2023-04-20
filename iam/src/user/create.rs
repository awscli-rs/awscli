use super::*;

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user to create.
    #[arg(long)]
    user_name: String,
}

#[async_trait]
impl Execute for CreateUser {
    async fn execute(self: Box<Self>, client: iam::Client) -> IamResult {
        let user = client
            .create_user()
            .user_name(self.user_name)
            .send()
            .await?
            .user;
        Ok(Box::new(user))
    }
}
