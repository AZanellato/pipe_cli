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
use indicatif::ProgressBar;
use itertools::any;
use quicli::prelude::*;
use std::borrow::Cow;
use structopt::StructOpt;

fn main() -> CliResult {
    loop {
        let cfg: ApiKey = load("pipe_cli")?;
        let working_cfg = match cfg.api_key {
            Some(key) => api_key::test_existing_api_key(key),
            None => api_key::get_working_api_key(),
        };
        println!("Hello {}! Welcome to Pipefy CLI", working_cfg.name);
        store("pipe_cli", &working_cfg)?;
        let args = args::Opts::from_args();
        let no_selection = !any(&[args.pipe_id, args.card_id], |id| id.is_some());

        if no_selection {
            let (selected_option, inputed_id) = main_select();
            // println!("option {} and id {}", selected_option, inputed_id);
            match selected_option {
                0 => pipe_sub_select(),
                1 => org_sub_select(),
                2 => card_sub_select(&working_cfg.api_key.unwrap(), inputed_id),
                _ => break,
            };
        }
        ProgressBar::new_spinner();
    }
    Ok(())
}

fn main_select<'a>() -> (i32, i32) {
    let selections = &["💈 Pipe", "🏭 Company", "🃏 Card", "Exit"];

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

fn pipe_sub_select<'a>() -> (i32, i32) {
    let selections = &["Phases", "Cards"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    (select as i32, 0)
}
fn card_sub_select(api_key: &str, id: i32) -> (i32, i32) {
    if let Ok(card_title) = graphql::card_query_and_print(api_key, id) {
        println!("{}", card_title);
    } else {
        println!("Unauthorized");
    }
    (0, 0)
}
fn org_sub_select<'a>() -> (i32, i32) {
    let selections = &["💈 Pipe", "🏭 Company", "🃏 Card"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    (select as i32, 0)
}
