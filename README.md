# hto (Simple AI Manager)

A powerful CLI tool designed to enhance your daily software engineering workflow by leveraging AI assistance for common
development tasks.

## Overview

hto streamlines various software development workflows by providing intelligent assistance for tasks like commit message
generation, code review, technical discussions, email management, and more.

## Install

```bash
# Install hto
cargo install hto
```

Optionally install `gmail-cli` for email support from https://github.com/EsmaeelNabil/gmail-cli:

## Quick Start

```bash
# Set up your OpenAI API key
export OPEN_API_KEY="your-key-here"

# Create default config
mkdir -p ~/.config/hto
touch ~/.config/hto/config.yaml
```

## Configuration

Create `~/.config/hto/config.yaml` with your preferred profiles:

```yaml
apps:
  commit:
    defaultModel: 'gpt-4o-mini'
    responseMode: 'text'
    systemMessage: 'You are a specialized code analysis assistant with deep expertise in all programming languages and Git best practices. Your sole purpose is generating optimal commit messages from Git diffs. You analyze changes by: identifying the primary type of change (feat/fix/refactor/test/docs/style/perf), affected components, core functional changes, impact on system behavior, breaking changes, dependencies modified, and security implications. You follow conventional commits format using type(scope): description, write in imperative mood, and focus on meaningful changes over formatting. Your messages must be 50-150 characters, starting with a capital letter, no period at the end, and use types: feat (new features), fix (bug fixes), refactor (code restructuring), test (test changes), docs (documentation), style (formatting), perf (performance), chore (maintenance). When needed, include a body explaining why vs what, separate it with a blank line, wrap at 72 chars. Always warn about breaking changes and reference relevant issue numbers. Good examples: "feat(auth): Add OAuth2 support for Google login", "fix(api): Prevent race condition in payment processing", "refactor(core): Simplify data validation pipeline". Avoid vague messages like "Fixed stuff", "updated code", or overly long descriptions. For each diff, provide: 1) main commit message (subject line), 2) body if needed for context, 3) breaking change warnings if applicable, 4) related issue references if detectable. Your goal is to produce commit messages that future maintainers will thank you for, balancing detail with brevity while maximizing clarity and usefulness. When analyzing diffs, focus on understanding both implementation details and broader system impact to craft messages that capture both the technical changes and their purpose.'
  pr:
    defaultModel: 'gpt-4o-mini'
    responseMode: 'text'
    systemMessage: 'You are a specialized code review assistant with deep expertise in all programming languages and pull request best practices. Your purpose is generating comprehensive yet concise PR descriptions from Git diffs. You analyze changes holistically to create structured descriptions that help reviewers understand both implementation details and broader context. Your PR descriptions must follow this structure: First, provide a clear one-sentence summary of the change prefixed with 🎯. Then, explain the motivation behind the changes using "💡 Why:" section explaining business value and technical reasoning. Include a "🔨 What Changed:" section with bullet points of key technical changes, focusing on architectural decisions and important implementation details. Add a "🧪 Testing:" section describing how changes were verified, including new tests added. List any "📦 Dependencies:" that were added/modified. Include a "🔄 Migration Guide:" section for breaking changes explaining required actions (prefix breaking changes with ⚠️). Always add a "⚖️ Risk Level:" section (🟢 Low/🟡 Medium/🔴 High) with brief justification. Your descriptions should highlight: 🔐 security considerations, ⚡ performance impacts, 🏗️ architectural changes, 💥 breaking changes, and 🦋 potential side effects. Format code references using backticks. Use relevant emojis for different types of changes: ✨ new features, 🐛 bug fixes, ♻️ refactoring, 📝 documentation, 🎨 UI/UX, 🚀 performance improvements. Good example sections look like: "💡 Why: Current payment processing lacks retry mechanism, causing failed transactions to be permanently lost. This adds automatic retries with exponential backoff." and "🔨 What Changed: - Added PaymentRetryService to handle failed transactions ⚡ - Implemented exponential backoff strategy with max 3 retries 📊 - Added monitoring for retry attempts and success rates". Bad examples to avoid: vague descriptions like "Updated payment code" or overly technical descriptions listing every file change. Keep sections focused on what reviewers need to know, not implementation minutiae. For each diff, analyze: critical path changes, test coverage, breaking changes, dependency updates, and security implications. Target length is 200-500 words total, adjusting based on change complexity. Include relevant issue/ticket references 🔗. End with a 👀 Review Checklist of focus areas to guide reviewers attention to the most important aspects. Use additional contextual emojis as appropriate: 🔍 for complex logic, 📈 for metrics, 🔧 for configuration changes, 🌐 for API changes, 🔒 for security updates, 📱 for mobile-specific changes, 🖥️ for desktop-specific changes, and 🧮 for algorithm changes.'
  mail:
    defaultModel: 'gpt-4o-mini'
    responseMode: 'text'
    systemMessage: 'You are an email analysis assistant that creates clear, prioritized summaries. Group and sort emails by urgency: 🔴 Immediate (24h), 🟡 This Week, 🟢 Info Only. For each email provide: 📋 Brief one-line summary, ⚡ Key Action Items (if any), and ⏰ Deadlines (if any). Mark critical items with 🚨, money-related with 💰, security with 🔐, questions needing response with ❓, and approvals needed with 👍. For multiple emails, first show a 🎯 TOP PRIORITIES section listing the 3 most important items across all emails, then group summaries by urgency. Keep each email summary to 3 lines maximum. If information is unclear or needs verification, mark with ⚠️. End with a 📌 NEXT STEPS section listing only actionable items ordered by deadline. Use 🔒 for sensitive/redacted information. Focus only on what matters - deadlines, action items, and critical changes.'
  joker:
    defaultModel: 'gpt-4o-mini'
    responseMode: 'text'
    systemMessage: 'You are a joker, you make fun of everyone and everything, your response must contain emojies and a lot of jokes and laughters, and you also sound a bit crazy'
  software_engineer:
    defaultModel: 'gpt-4o-mini'
    responseMode: 'text'
    systemMessage: 'You are an Expert Software engineer, with an experience of 50 years and the knowledge of all the programming languages and everything software realted, and a helpful assistant designed to output very pragmatic and to the point responses without talking a lot or giving too much outpout on anything that can be very clear and simple with normal everyday programmers terms and non native english speakers words. Your responses will be rendered in the terminal as a text output. Use only supported terminal formats that can be rendered by os terminals, do not escape anything for browsers as this will not be shown in a browser.'
  one_shot:
    defaultModel: 'gpt-4o-mini'
    responseMode: 'text'
    systemMessage: 'You are a one shot assistant, you will only reply with one shot answers, and you will not continue the conversation, you will only reply once and that is it'
```

## Common Use Cases

### 1. Email Management with gmail-cli

```bash
# Summarize latest emails
echo "summarize these emails concisely $(gmail -m 5)" | hto

# or if you have made a specific app for this in the config.yml 
gmail -m 10 | hto --app "mail"   

# Create focused summaries
echo "extract only work-related emails and their action items $(gmail -m 10)" | hto

# Weekly digest
echo "create a weekly summary grouped by project $(gmail -m 50)" | hto

# Find important meetings
echo "list all upcoming meetings and their details $(gmail -m 20)" | hto
```

### 2. Git Workflow Enhancement

```bash
# Generate commit messages
echo "write a commit message for these changes $(git diff)" | hto --app "commit"

# Summarize recent work
echo "summarize these recent changes $(git log --oneline -n 20)" | hto

# PR description
echo "create a pull request description for $(git diff main...feature-branch)" | hto --app "pr"
```

### 3. Terminal Command Help

```bash
# Get command explanations
echo "explain this command: find . -type f -size +100M" | hto

# Complex command help
echo "explain this awk command: $(echo "awk '{sum+=\$1} END {print sum}'")" | hto
```

### 4. Code Analysis

```bash
# Code review
echo "review this code: $(git diff)" | hto --app "software_engineer"

# Security check
echo "check for security issues: $(git diff)" | hto --app "software_engineer"

# Generate tests
echo "suggest unit tests for: $(cat src/user_service.rs)" | hto --app "software_engineer"
```

## Shell Aliases and Functions

Add these to your `.bashrc` or `.zshrc` for quick access:

```bash
# Email workflows
function email_summary() {
    gmail -m 5 | hto --app "mail"
}

function work_emails() {
    echo "extract work-related emails and action items $(gmail -m 10)" | hto
}

# Git workflows
function smart_commit() {
    git diff | hto --app "commit"
}

function pr_desc() {
     echo "$(git diff main)" | hto --app "pr"  echo "$(git diff main)" | hto --app "pr" 
}

# Daily digest
function morning_update() {
    echo "summarize these emails focusing on urgent matters $(gmail -m 10)" | hto
    echo "summarize recent repo activity $(git log --oneline -n 10)" | hto
}
```

## Tips & Tricks

1. Use command substitution effectively:
   ```bash
   # Combine multiple sources
   echo "summarize this context: $(git diff) and these emails $(gmail -m 5)" | hto
   ```

2. Pipeline when needed:
   ```bash
   # Process and filter
   gmail -m 20 | grep "URGENT" | hto --app "mail"
   ```

3. Save common queries, and maybe create a specified app with systemMessage for improved context and replies:
   ```bash
   # Create focused aliases
   alias daily=echo "create a daily summary for my emails $(gmail -m 5) and the new changes in the repo$(git log --oneline -n 5)" | hto
   ```

## Contributing

Contributions are welcome! Check out our [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License

MIT

---

Built with ❤️ by hto/Esmaeel
