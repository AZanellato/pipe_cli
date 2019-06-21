use crate::graphql::me_query;
use dialoguer::{theme::ColorfulTheme, PasswordInput};
use serde::{Deserialize, Serialize};
use std::{error, fmt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub api_key: Option<String>,
    pub name: String,
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

impl ::std::default::Default for ApiKey {
    fn default() -> Self {
        Self {
            api_key: None,
            name: "".into(),
        }
    }
}

pub fn test_existing_api_key(api_key: String) -> ApiKey {
    match test_api_key(api_key) {
        Ok(api_key) => api_key,
        Err(_) => {
            println!("Your API key is invalid, please update it");
            get_working_api_key()
        }
    }
}
pub fn get_working_api_key() -> ApiKey {
    let api_key = get_api_key();
    match test_api_key(api_key) {
        Ok(api_key_name) => api_key_name,
        Err(_) => {
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

fn test_api_key(api_key: String) -> Result<ApiKey, InvalidAPIKey> {
    let result = me_query(&api_key);
    match result {
        Ok(name) => Ok(ApiKey {
            name: name,
            api_key: Some(api_key),
        }),
        Err(e) => {
            println!("{:?}", e);
            Err(InvalidAPIKey::new())
        }
    }
}
