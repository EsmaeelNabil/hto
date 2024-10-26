## Overview

This tool is designed for interacting with OpenAi LLMs through a terminal interface. It allows you to
configure various parameters and get responses from the model.

## Install

```bash
cargo install hto
```

## Configuration

- Create a configuration file in `~/.config/hto/config.yaml` The file format should be YAML.
- Example `config.yaml`:

```yaml
    apps:
      one_shot:
        defaultModel: "gpt-4o-mini"
        responseMode: "text"
        systemMessage: "You are an extremely helpful assistant."
```

- The application will use this file on startup. If the file does not exist, default settings will be applied.
