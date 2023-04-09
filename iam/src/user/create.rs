use super::*;

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user to create.
    #[arg(long)]
    user_name: String,
}

impl CreateUser {
    pub async fn execute(self, client: iam::Client) -> IamResult<Option<User>> {
        let user = client
            .create_user()
            .user_name(self.user_name)
            .send()
            .await?
            .user;
        Ok(user)
    }
}
