use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::{error, fmt};

#[derive(Debug, Clone)]
struct Unauthorized;

impl Unauthorized {
    fn new() -> Unauthorized {
        Unauthorized {}
    }
}

impl fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized access")
    }
}

impl error::Error for Unauthorized {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
pub fn perform_me_query(api_key: &str) -> Result<String, Box<Error>> {
    let mut map = HashMap::new();
    map.insert("query", "query { me { name } }");
    let client = reqwest::Client::new();
    let mut res = client
        .post("https://app.pipefy.com/queries")
        .json(&map)
        .bearer_auth(api_key)
        .send()?;

    let json_as_text = res.text()?;
    println!("response text: {:?}", json_as_text);
    let response_body: Value = serde_json::from_str(&json_as_text)?;
    let name = &response_body["data"]["me"]["name"];
    match name {
        serde_json::Value::String(name) => Ok(name.to_string()),
        _ => Err(Box::new(Unauthorized::new())),
    }
}
