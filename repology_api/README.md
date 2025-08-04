Of course. A good `README.md` is essential for any project. Here is a comprehensive README file for your `repology-cli` tool, written in Markdown.

You can copy and paste this directly into a new file named `README.md` in your project's root directory.

---

# Repology CLI

![License](https://img.shields.io/badge/license-MIT-blue.svg)![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

A modern, fast, and feature-rich command-line interface to query the [Repology.org](https://repology.org) API. Find out where your favorite software packages are, what versions are available, and their update status across hundreds of software repositories.

## Demo



## Features

*   **Direct API Query**: Quickly fetch package information right from your terminal.
*   **Tabular Output**: Displays results in a clean, human-readable table.
*   **Automatic Paging**: Long results are automatically piped into `less` for easy navigation.
*   **Pre-emptive Filtering**: Filter by distribution/repository *before* displaying results.
*   **Configurable Timeout**: Set a custom network timeout for slow connections or large queries.
*   **Shell Completion**: Generate autocompletion scripts for Bash, Zsh, Fish, and other shells.
*   **Robust Error Handling**: Built to handle network errors, API issues, and user interruptions gracefully.

## Installation

### Prerequisites

You must have the Rust programming language toolchain installed. You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### From Crates.io (Recommended)

Once the package is published to `crates.io`, it can be installed with a single command:
```bash
# This will be available once the crate is published
cargo install repology-cli
```

### From Source (Current Method)

Clone the repository and install the binary using `cargo`:
```bash
git clone <your-repo-url>
cd repology-cli
cargo install --path .
```
This command builds the project and copies the `repology-cli` binary to `~/.cargo/bin`, which should be in your system's `PATH`.

## Usage

The tool is structured with subcommands. The primary commands are `query` and `generate-completion`.

### 1. Querying for Packages

The `query` command is used to fetch and display package information.

**Basic Query:**

```bash
repology-cli query <PACKAGE_NAME>
```
*Example:*

```bash
repology-cli query ripgrep
```

**Filtering by Distribution:**
You can filter results to only show repositories that contain a specific string (case-insensitive).

```bash
repology-cli query <PACKAGE_NAME> --distro <DISTRO_NAME>
# or using the short flag
repology-cli query <PACKAGE_NAME> -d <DISTRO_NAME>
```

*Example:*

```bash
repology-cli query ripgrep --distro arch
```

**Handling Timeouts:**
For very large projects (like `rust` or `python`), the API may take a while to respond. You can increase the timeout from the default of 10 seconds.

```bash
repology-cli query <PACKAGE_NAME> --timeout <SECONDS>
```
*Example:*

```bash
repology-cli query rust --timeout 30
```

### 2. Generating Shell Completions

To make the tool even easier to use, you can generate a completion script for your shell. This allows you to autocomplete commands, flags, and arguments by pressing the `Tab` key.

**Note:** The setup is a **one-time action**.

#### For Bash:

1.  Add the completion script generation to your `~/.bashrc` file.
    ```bash
    repology-cli generate-completion bash >> ~/.bashrc
    ```
2.  Reload your shell's configuration.
    ```bash
    source ~/.bashrc
    ```

#### For Zsh:

1.  Create a dedicated directory for Zsh completions if you don't have one.
    ```bash
    mkdir -p ~/.zsh/completion
    ```
2.  Add this directory to your `fpath` in `~/.zshrc` and initialize `compinit`. Make sure these lines are in your `.zshrc`:
    ```zsh
    # Add this to your ~/.zshrc
    fpath=($HOME/.zsh/completion $fpath)
    autoload -U compinit
    compinit
    ```
3.  Generate the completion script into the completions directory.
    ```bash
    repology-cli generate-completion zsh > ~/.zsh/completion/_repology-cli
    ```
4.  Reload your shell's configuration.
    ```bash
    source ~/.zshrc
    ```

Once set up, you can type `repology-cli` and press `Tab` to see available commands.

## Development

To build the project from source for development purposes:
```bash
# Clone the repository
git clone <your-repo-url>
cd repology-cli

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run -- query ripgrep
```

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.