use clap::Parser;
use std::io;
use std::io::Read;

pub const DEFAULT_QUERY: &str =
    "reply with this and nothing more: please provide a --query 'your query'";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct H2oArgs {
    /// Name of the person to greet
    #[arg(index = 1, default_value = DEFAULT_QUERY)]
    pub query: String,

    /// Name of the person to greet
    #[arg(short, long, default_value = "one_shot")]
    pub app: String,

    /// Name of the person to greet
    #[arg(short, long, default_value = "gpt-4o-mini")]
    pub model: String,

    /// Name of the person to greet
    #[arg(short, long, default_value = ".config/h2o/config.yaml")]
    pub config: String,

    /// Enable debug mode
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
