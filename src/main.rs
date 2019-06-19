extern crate dialoguer;
extern crate indicatif;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate structopt;
mod api_key;
mod args;
mod graphql;
use api_key::ApiKey;
use confy::{load, store};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use itertools::any;
use quicli::prelude::*;
use structopt::StructOpt;

fn main() -> CliResult {
    let cfg: ApiKey = load("pipe_cli")?;
    let working_cfg = match cfg.api_key {
        Some(key) => api_key::test_existing_api_key(key),
        None => api_key::get_working_api_key(),
    };
    println!("Hello {}! Welcome to Pipefy CLI", working_cfg.name);
    store("pipe_cli", &working_cfg)?;
    let api_key = &working_cfg.api_key.unwrap();
    let args = args::Opts::from_args();
    let no_selection = !any(&[args.pipe_id, args.card_id], |id| id.is_some());
    loop {
        if no_selection {
            let (selected_option, inputed_id) = main_select();
            match selected_option {
                0 => pipe_sub_select(&api_key, inputed_id),
                1 => organization_sub_select(&api_key, inputed_id),
                2 => card_sub_select(&api_key, inputed_id),
                _ => break,
            };
        }
    }
    bye();
    Ok(())
}

fn main_select<'a>() -> (i32, i32) {
    let selections = &["💈 Pipe", "🏭 Organization", "🃏 Card", "Exit"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what you want to see")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    if select == 3 {
        return (3, 0);
    }
    let input: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("The ID, please")
        .interact()
        .unwrap();

    (select as i32, input)
}

fn pipe_sub_select<'a>(api_key: &str, pipe_id: i32) -> () {
    if let Err(_) = graphql::pipe_name_query(api_key, pipe_id) {
        println!("Unauthorized");
        return ();
    }
    let selections = &["Phases", "Cards"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match select {
        0 => {
            if let Err(_) = graphql::pipe_phases_query(api_key, pipe_id) {
                println!("Unauthorized");
            }
        }
        1 => {
            if let Err(_) = graphql::pipe_cards_query(api_key, pipe_id) {
                println!("Unauthorized");
            }
        }
        _ => {
            println!("Invalid option");
        }
    }
}
fn card_sub_select(api_key: &str, id: i32) -> () {
    if let Err(_) = graphql::card_query_and_print(api_key, id) {
        println!("Unauthorized");
    }
}
fn organization_sub_select<'a>(api_key: &str, company_id: i32) -> () {
    if let Err(_) = graphql::organization_name_query(api_key, company_id) {
        println!("Unauthorized");
        return ();
    }
    let selections = &["💈💈 Pipes 💈💈", "👥👥 Members 👥👥"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if let Err(_) = graphql::card_query_and_print(api_key, company_id) {
        println!("Unauthorized");
    }
    (0, 0)
}
