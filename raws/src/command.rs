use super::*;

#[derive(Debug, Subcommand)]
pub enum Command {
    Dynamodb,
    Ec2,
    Eks,
}

impl Command {
    pub async fn dispatch(self) -> miette::Result<()> {
        match self {
            Self::Dynamodb => Ok(()),
            Self::Ec2 => Ok(()),
            Self::Eks => Ok(()),
        }
    }
}
