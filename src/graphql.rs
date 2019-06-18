use prettyprint::PrettyPrinter;
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
pub fn me_query(api_key: &str) -> Result<String, Box<Error>> {
    let mut query = HashMap::new();
    query.insert("query", String::from("query { me { name } }"));
    let text_response = perform_query(api_key, query)?;
    let response_body: Value = serde_json::from_str(&text_response)?;
    let name = &response_body["data"]["me"]["name"];
    match name {
        serde_json::Value::String(name) => Ok(name.to_string()),
        _ => Err(Box::new(Unauthorized::new())),
    }
}

pub fn card_query_and_print(api_key: &str, card_id: i32) -> Result<(), Box<Error>> {
    let print = PrettyPrinter::default()
        .language("rust")
        .grid(true)
        .line_numbers(true)
        .build()
        .unwrap();

    let mut query: HashMap<&str, String> = HashMap::new();
    let format_card_query_string = format!(
        "query {{
        card(id: {id}) {{
            title
            fields {{
                value
                name
    }} }} }}",
        id = card_id
    );
    let card_query_string = String::from(format_card_query_string);
    query.insert("query", card_query_string);
    let text_response = perform_query(api_key, query)?;
    let response_body: Value = serde_json::from_str(&text_response)?;
    match &response_body["data"]["card"]["title"] {
        serde_json::Value::String(response) => {
            let field_values =
                serde_json::to_string_pretty(&response_body["data"]["card"]["fields"]).unwrap();
            print.string_with_header(field_values, response.to_string())?;
            Ok(())
        }
        _ => Err(Box::new(Unauthorized::new())),
    }
}

pub fn pipe_name_query(api_key: &str, pipe_id: i32) -> Result<(), Box<Error>> {
    let print = PrettyPrinter::default()
        .language("rust")
        .grid(true)
        .line_numbers(true)
        .build()
        .unwrap();
    let mut query: HashMap<&str, String> = HashMap::new();
    let format_pipe_query_string = format!(
        "query {{
        pipe(id: {id}) {{
            name
    }} }}",
        id = pipe_id
    );
    let pipe_query_string = String::from(format_pipe_query_string);
    query.insert("query", pipe_query_string);
    let text_response = perform_query(api_key, query)?;
    let response_body: Value = serde_json::from_str(&text_response)?;
    match &response_body["data"]["pipe"]["name"] {
        serde_json::Value::String(response) => {
            print.string_with_header(response.to_string(), "Pipe Name".to_string())?;
            Ok(())
        }
        _ => Err(Box::new(Unauthorized::new())),
    }
}

pub fn pipe_phases_query(api_key: &str, pipe_id: i32) -> Result<(), Box<Error>> {
    let print = PrettyPrinter::default()
        .language("rust")
        .grid(true)
        .line_numbers(true)
        .build()
        .unwrap();
    let mut query: HashMap<&str, String> = HashMap::new();
    let format_pipe_query_string = format!(
        "query {{
        pipe(id: {id}) {{
            phases {{ name cards_count description }}
    }} }}",
        id = pipe_id
    );
    let pipe_query_string = String::from(format_pipe_query_string);
    query.insert("query", pipe_query_string);
    let text_response = perform_query(api_key, query)?;
    let response_body: Value = serde_json::from_str(&text_response)?;
    match &response_body["data"]["pipe"]["phases"] {
        serde_json::Value::Array(_) => {
            let phases =
                serde_json::to_string_pretty(&response_body["data"]["pipe"]["phases"]).unwrap();
            print.string_with_header(phases, "Phases".to_string())?;
            Ok(())
        }
        _ => Err(Box::new(Unauthorized::new())),
    }
}

fn perform_query(api_key: &str, query: HashMap<&str, String>) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut res = client
        .post("https://app.pipefy.com/queries")
        .json(&query)
        .bearer_auth(api_key)
        .send()?;

    res.text()
}
