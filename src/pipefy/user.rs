use crate::graphql::me_query;
use dialoguer::{theme::ColorfulTheme, PasswordInput};
use serde::{Deserialize, Serialize};
use std::{error, fmt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub api_key: String,
    pub info: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub name: String,
    #[serde(deserialize_with = "crate::graphql::from_str")]
    pub id: u32,
}

#[derive(Debug, Clone)]
struct InvalidAPIKey;

impl InvalidAPIKey {
    fn new() -> InvalidAPIKey {
        InvalidAPIKey {}
    }
}

impl fmt::Display for InvalidAPIKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidAPIKey used")
    }
}
impl error::Error for InvalidAPIKey {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl ::std::default::Default for User {
    fn default() -> Self {
        Self {
            api_key: "".into(),
            info: UserInfo {
                name: "".into(),
                id: 0,
            },
        }
    }
}

pub fn test_existing_api_key(user: User) -> User {
    match test_api_key(user.api_key) {
        Ok(user) => user,
        Err(e) => {
            println!("{}", e);
            println!("Your API key is invalid, please update it");
            get_working_api_key()
        }
    }
}
pub fn get_working_api_key() -> User {
    let api_key = get_api_key();
    match test_api_key(api_key) {
        Ok(user) => user,
        Err(e) => {
            println!("{}", e);
            println!("Invalid API key, please try again");
            get_working_api_key()
        }
    }
}

fn get_api_key() -> String {
    let new_api_key = PasswordInput::with_theme(&ColorfulTheme::default())
        .with_prompt("Your API key is not defined, please type it")
        .interact()
        .unwrap();
    new_api_key.into()
}

fn test_api_key(api_key: String) -> Result<User, InvalidAPIKey> {
    let result = me_query(&api_key);
    match result {
        Ok(api_key) => Ok(api_key),
        Err(e) => {
            println!("{:?}", e);
            Err(InvalidAPIKey::new())
        }
    }
}
