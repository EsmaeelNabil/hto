mod cli;
mod config;
mod openapi;

use clap::Parser;
use cli::get_query;
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

    // get application from config yaml file.
    let app = config.apps.get(args_app);

    match app {
        Some(app) => {
            let response_mode = app.responseMode.as_str();
            let model = app.defaultModel.as_str();

            match openapi::get_api_response(
                query.as_str(),
                model,
                app.systemMessage.as_str(),
                response_mode,
                api_key.as_str(),
                args.debug,
            )
            .await
            {
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
        None => {
            eprintln!(
                "App: {} not found, make sure ~/.config/hto/config.yaml exists, and has this app",
                args_app
            );
            std::process::exit(1);
        }
    }
}
