mod openapi;
mod config;
mod cli;

use cli::get_query;
use clap::Parser;
use cli::H2oArgs;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();
    let api_key = env::var("OPEN_API_KEY").expect("export OPEN_API_KEY first please");

    let args = H2oArgs::parse();
    // Determine if query should be read from stdin
    let query = get_query(&args);
    let args_app = args.app.as_str();

    if args.debug {
        println!("Query: {:?}", query);
        println!("App: {:?}", args.app);
        println!("Config: {:?}", args.config);
    }

    let config = config::get_config(&args);

    // application configuration
    let app = config.apps.get(args_app).unwrap();
    let response_mode = app.responseMode.as_str();
    let model = app.defaultModel.as_str();

    match openapi::get_api_response(
        query.as_str(),
        model,
        app.systemMessage.as_str(),
        response_mode,
        api_key.as_str(),
        args.debug,
    ).await {
        Ok(response) => {
            if let Some(actual_string) = response.as_str() {
                println!("{}", actual_string);
            } else {
                // or handle differently for function calling in case of json
                println!("{}", response);
            }
        }
        Err(err) => eprintln!("Something wrong happened!: {}", err),
    }
}

