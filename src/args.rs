use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Args", about = "Possible arguments")]
pub struct Opts {
    // Which card to get
    #[structopt(long = "card", short = "c")]
    card_id: Option<usize>,
    // Which pipe to get
    #[structopt(long = "pipe", short = "p")]
    pipe_id: Option<usize>,
}
