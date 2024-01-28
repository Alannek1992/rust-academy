use anyhow::Result;
use common::{connection::frame::auth::Username, util};
use dialoguer::Select;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub struct Cmd;

pub struct AuthInput {
    pub username: String,
    pub password: String,
    pub existing_account: bool,
}

impl Cmd {
    pub fn new() -> Self {
        Self
    }

    pub async fn read_auth_details() -> Result<AuthInput> {
        util::print_msg_to_stdout(
            "Welcome to CHAT client. Use your account to continue or create one",
            util::ColorFacade::Yellow,
        );

        let items = vec!["REGISTER", "LOGIN"];
        let selection = Select::new().items(&items).interact()?;

        let existing_account = items[selection] == items[1];

        util::print_msg_to_stdout("Enter your username:", util::ColorFacade::Yellow);
        let username = Self::read_user_input::<Username>().await?;
        util::print_msg_to_stdout("Enter your password:", util::ColorFacade::Yellow);
        let password = Self::read_user_input::<String>().await?;

        Ok(AuthInput {
            username,
            password,
            existing_account,
        })
    }

    pub async fn read_user_message() -> Result<String> {
        Self::read_user_input::<String>().await
    }

    async fn read_user_input<T: Into<String>>() -> Result<String> {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        // Read one line from stdin
        let mut input = String::new();
        reader.read_line(&mut input).await?;
        Ok(input)
    }
}
