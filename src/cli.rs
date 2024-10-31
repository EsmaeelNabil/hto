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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_query() {
        let args = H2oArgs::parse_from(&["hto"]);
        assert_eq!(args.query, DEFAULT_QUERY);
    }

    #[test]
    fn test_custom_app_argument() {
        let args = H2oArgs::parse_from(&["hto", "--app", "software_engineer"]);
        assert_eq!(args.app, "software_engineer");
    }

    #[test]
    fn test_model_argument() {
        let args = H2oArgs::parse_from(&["hto", "--model", "gpt-3.5"]);
        assert_eq!(args.model, "gpt-3.5");
    }

    #[test]
    fn test_invalid_argument() {
        let result = H2oArgs::try_parse_from(&["hto", "--unknown", "value"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_debug_mode() {
        let args = H2oArgs::parse_from(&["hto", "-d"]);
        assert!(args.debug);
    }
}
