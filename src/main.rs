extern crate serde;
extern crate serde_json;
extern crate structopt;
mod api_key;
mod args;
mod graphql;
use api_key::ApiKey;
use confy::{load, store};
use quicli::prelude::*;
use structopt::StructOpt;

fn main() -> CliResult {
    // let cfg: ApiKey = confy::load("pipe_cli")?;
    let working_cfg = api_key::test_existing_api_key("".into());
    // let working_cfg = match cfg.api_key {
    //     Some(key) => api_key::test_existing_api_key(key),
    //     None => api_key::get_working_api_key(),
    // };
    println!("{:?}", working_cfg);
    confy::store("pipe_cli", working_cfg.clone())?;
    println!("Hello {}! Welcome to Pipefy CLI", working_cfg.name);
    let args = args::Opts::from_args();
    Ok(())
}
