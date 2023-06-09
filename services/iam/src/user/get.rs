use super::*;

#[derive(Debug, Args)]
pub struct GetUser {
    /// The name of the user to get information about.
    #[arg(long)]
    user_name: String,
}

#[async_trait]
impl Execute for GetUser {
    async fn execute(self: Box<Self>, client: iam::Client) -> IamResult {
        let user = client
            .get_user()
            .user_name(self.user_name)
            .send()
            .await?
            .user;
        Ok(Box::new(user))
    }
}
