extern crate dialoguer;
extern crate indicatif;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate structopt;
mod args;
mod pipefy;
use confy::{load, store};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use itertools::any;
use pipefy::{graphql, user};
use prettyprint::PrettyPrinter;
use quicli::prelude::*;
use structopt::StructOpt;
use user::User;

fn main() -> CliResult {
    welcome();
    let stored_user = load::<User>("pipe_cli");
    let user = match stored_user {
        Ok(user) => user::test_existing_api_key(user),
        _ => user::get_working_api_key(),
    };
    println!("Hello {}! Welcome to Pipefy CLI", user.info.name);
    store("pipe_cli", &user)?;
    let api_key = &user.api_key;
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
        } else {
            if args.pipe_id.is_some() {
                pipe_sub_select(&api_key, args.pipe_id.unwrap());
            } else if args.card_id.is_some() {
                card_sub_select(&api_key, args.card_id.unwrap());
                break;
            }
        };
    }
    bye();
    Ok(())
}

fn main_select<'a>() -> (usize, usize) {
    let selections = &["💈 Pipe", "🏭 Organization", "🃏 Card", "Exit"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what you want to see")
        .default(0)
        .items(selections)
        .paged(true)
        .interact()
        .unwrap();
    if select == 3 {
        return (3, 0);
    }
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("The ID, please")
        .interact()
        .unwrap();

    (select, input)
}

fn pipe_sub_select<'a>(api_key: &str, pipe_id: usize) -> () {
    if let Err(_) = graphql::pipe_name_query(api_key, pipe_id as i32) {
        println!("Unauthorized");
        return ();
    }
    let selections = &[
        "See All Phases",
        "See All Cards",
        "Select One Phase",
        "Select One Card",
    ];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();

    match select {
        0 => {
            if let Err(_) = graphql::pipe_phases_query(api_key, pipe_id as i32) {
                println!("Unauthorized");
            }
        }
        1 => {
            if let Err(_) = graphql::pipe_cards_query(api_key, pipe_id as i32) {
                println!("Unauthorized");
            }
        }

        2 => {
            phases_pipe_selection(api_key, pipe_id);
        }

        3 => {
            cards_pipe_selection(api_key, pipe_id as i32);
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn phases_pipe_selection(api_key: &str, pipe_id: usize) -> () {
    //TODO
    //Make it actually do something here 😅
    let selections = &["See all", "Select one"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();
}
fn cards_pipe_selection(api_key: &str, pipe_id: i32) -> () {
    let cards = graphql::pipe_cards_select(api_key, pipe_id).unwrap();

    let card_selection: Vec<String> = cards
        .iter()
        .map(|card_node| card_node.node.title.to_string())
        .collect();

    let card_select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which card?")
        .default(0)
        .paged(true)
        .items(&card_selection[..])
        .interact()
        .unwrap();

    let print = PrettyPrinter::default()
        .language("rust")
        .grid(true)
        .line_numbers(true)
        .build()
        .unwrap();

    let card_node = cards.get(card_select).unwrap();
    print
        .string_with_header(card_node.node.to_string(), "Card".to_string())
        .expect("Something went wrong printing the Card");
}

fn card_sub_select(api_key: &str, id: usize) -> () {
    if let Err(_) = graphql::card_query_and_print(api_key, id as i32) {
        println!("Unauthorized");
    }
}
fn organization_sub_select<'a>(api_key: &str, company_id: usize) -> () {
    if let Err(_) = graphql::organization_name_query(api_key, company_id as i32) {
        println!("Unauthorized");
        return ();
    }
    let selections = &["💈💈 Pipes 💈💈", "👥👥 Members 👥👥"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to see?")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();
    match select {
        0 => {
            if let Err(_) = graphql::org_pipes_query(api_key, company_id as i32) {
                println!("Unauthorized");
            }
        }
        1 => {
            if let Err(_) = graphql::org_members_query(api_key, company_id as i32) {
                println!("Unauthorized");
            }
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn welcome() -> () {
    println!("
                                                                      `-/+o
                                                                     /ssso+
                                                                    /sss-
          ---``-:///:.     `---` .--.`-:///:-`        .-////:.    .:ssso------`     .---.
         `ossssssssssss/`  -sss. +ssssssssssss+.    -osssssssss/` +ssssssssssso.   :sss+
         `ossso:.``.:ssss. -sss. +ssss/.``.:osss-  +sss/.``.:osso.`.sss+```.osss. -sss/
         `osso`      `sss+ -sss. +sss.       +sss`-sss+///////sss+ `sss/    `osss-`+s/
         `oss+        osso -sss. +sss`       /sss`:sssssssssssssso `sss/     `+sss-`-
         `osss/`    `/sss: -sss. +sss+.    `:sss+ `osso.    `....  `sss/      `ssss`
         `ossssso++ossso-  -sss. +ssssso++ossss/   `+sssso+ossso-  `sss/     `osss-
         `oss+:+ossso/-    -ooo. +sss:+ossso+:`      .:+ossso/-    `ooo/    `osss.
         `oss/                   +sso                                      .osso.
         `oss/                   +sso                                     .ssso`
          .--.                   .--.                                     .---`
                                                                                                    ");
}

fn bye() -> () {
    println!(
        "
        88
        88
        88
        88,dPPYba,  8b       d8  ,adPPYba,
        88P'    ''8a `8b     d8' a8P_____88
        88       d8  `8b   d8'  8PP'''''''
        88b,   ,a8''   `8b,d8'   ''8b,   ,aa
        8Y'Ybbd8'''      Y88'    `''Ybbd8'''
                        d8'
                    d8'
       "
    );
}
