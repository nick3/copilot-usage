# GitHub Copilot Usage Viewer (Rust Version)

This command-line tool fetches and displays GitHub Copilot seat usage for a specified organization. It is a Rust rewrite of the original TypeScript version, designed for improved performance and reliability.

 [🇨🇳 中文文档](./README-zh.md)
## Features

-   Fetches all assigned Copilot seats for a GitHub organization.
-   Displays usage details in a clean, readable table format.
-   Calculates and shows the number of active vs. total seats.
-   Handles GitHub API pagination automatically.

## Prerequisites

-   Rust and Cargo (latest stable version recommended)
-   A GitHub Personal Access Token (PAT) with `read:org` scope.

## Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/nick3/copilot-usage.git
    cd copilot-usage
    ```

2.  **Create an environment file:**
    Create a file named `.env` in the project root and add your GitHub Personal Access Token to it:

    ```
    GITHUB_PAT=your_github_personal_access_token_here
    ```

## Usage

### Running the application

You can run the application directly using Cargo. Replace `<your_org>` with the name of your GitHub organization.

```bash
cargo run --organization <your_org>
```

Alternatively, you can use the short flag `-o`:

```bash
cargo run -o <your_org>
```

### Building the application

To build a release-optimized binary, run:

```bash
cargo build --release
```

The executable will be located at `target/release/rust_rewrite`. You can then run it directly:

```bash
./target/release/rust_rewrite -o <your_org>
```

### Testing

To run the test suite:

```bash
cargo test
```
