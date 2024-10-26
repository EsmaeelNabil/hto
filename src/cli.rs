use clap::Parser;
use std::io;
use std::io::Read;

pub const DEFAULT_QUERY: &str =
    "reply with this and nothing more: please provide a --query 'your query'";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct H2oArgs {
    /// Query to send to the model
    #[arg(index = 1, default_value = DEFAULT_QUERY)]
    pub query: String,

    /// App to use from the config file at ~/.config/hto/config.yaml
    #[arg(short, long, default_value = "one_shot")]
    pub app: String,

    /// OpenAI Model to use, default is gpt-4o-mini
    #[arg(short, long, default_value = "gpt-4o-mini")]
    pub model: String,

    /// Config file to use, default is ~/.config/hto/config.yaml
    #[arg(short, long, default_value = ".config/hto/config.yaml")]
    pub config: String,

    /// Enable debug mode for more verbose output
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}

pub fn get_query(args: &H2oArgs) -> String {
    let query = if &args.query == DEFAULT_QUERY && !atty::is(atty::Stream::Stdin) {
        read_from_stdin()
    } else {
        args.query.clone()
    };
    query
}

pub fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");
    buffer
}
