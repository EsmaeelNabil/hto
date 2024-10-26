# LLM Application

## Overview

This tool is designed for interacting with OpenAi LLMs through a terminal interface. It allows you to
configure various parameters and get responses from the model.

## Features

- **Configurable Models**: Choose default models and response modes.
- **Easy Setup**: Load settings from a YAML config file or use default settings.
- **Progress Feedback**: Displays a spinner while waiting for API responses.

## Download

## Configuration

- Create a configuration file in `~/.config/h2o/config.yaml` The file format should be YAML.
- Example `config.yaml`:

```yaml
    apps:
      one_shot:
        defaultModel: "gpt-4o-mini"
        responseMode: "text"
        systemMessage: "You are an extremely helpful assistant."
```

- The application will use this file on startup. If the file does not exist, default settings will be applied.

## Usage

Run the application with the following command:

```bash
cargo run -- --config <your_config_file_path.yaml> 'Your query here'
```

### Debugging

To enable debug mode (show more output):

```bash
h2o 'Your query here' -d
```