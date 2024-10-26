use crate::cli::H2oArgs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LLMApp {
    pub defaultModel: String,
    pub responseMode: String,
    pub systemMessage: String,
}

impl Default for LLMApp {
    fn default() -> Self {
        LLMApp {
            defaultModel: "gpt-4o-mini".to_string(),
            responseMode: "text".to_string(),
            systemMessage: "You are an extremely helpful assistant that is designed to be used inside the terminal, reply with this and nothing more: please provide a --query 'your query'".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct H2oConfig {
    pub apps: HashMap<String, LLMApp>,
}

impl Default for H2oConfig {
    fn default() -> Self {
        let mut apps = HashMap::new();
        apps.insert("one_shot".to_string(), LLMApp::default());

        H2oConfig { apps }
    }
}

/**
* Read the content of a file
*/
fn read_yaml_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/**
* Get the configuration from the config file
* If the config file doesn't exist, return the default configuration
*/
pub fn get_config(args: &H2oArgs) -> H2oConfig {
    let config_path = dirs::home_dir().unwrap().join(&args.config);
    if Path::new(config_path.as_path()).exists() {
        // read config
        let config_file_content = read_yaml_file(config_path.to_str().unwrap()).unwrap();
        // return config
        serde_yml::from_str(&config_file_content).unwrap()
    } else {
        // return default config
        H2oConfig::default()
    }
}
