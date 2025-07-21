# Design Document: Rust Rewrite

## 1. Overview

This document details the technical design for rewriting the `GithubCopilotUsages` command-line tool from TypeScript to Rust. The goal is to create a more performant, memory-safe, and robust application by leveraging Rust's strengths, while maintaining full functional parity with the original tool.

## 2. Architecture

The Rust application will follow a modular architecture similar to the original TypeScript project, promoting separation of concerns. The project will be a binary crate managed by Cargo.

The proposed module structure is as follows:

-   `src/main.rs`: The application's entry point. Responsible for command-line argument parsing, environment variable loading, and orchestrating calls to other modules.
-   `src/github_client.rs`: Handles all communication with the GitHub API. Responsible for making HTTP requests, handling pagination, and deserializing API responses.
-   `src/processor.rs`: Contains the business logic for processing the raw data fetched from the API into a more usable format.
-   `src/formatter.rs`: Responsible for presenting the processed data to the user in a clean, tabular format in the console.
-   `src/types.rs`: Defines the core data structures used throughout the application.
-   `src/error.rs`: Defines custom error types for robust error handling.

The application will be built on the `tokio` asynchronous runtime to handle concurrent API requests efficiently.

## 3. Components and Interfaces (Crate Dependencies)

The following Rust crates will be used to replace the npm packages from the original project:

-   **`tokio`**: The asynchronous runtime for Rust. It will be used for all async operations.
-   **`reqwest`**: A high-level HTTP client. It will replace `axios` for making requests to the GitHub API.
-   **`clap`**: A powerful command-line argument parser. It will replace `yargs`.
-   **`serde` & `serde_json`**: For serializing and deserializing data, primarily for handling JSON responses from the GitHub API.
-   **`chrono`**: A library for date and time handling, necessary for processing the `last_activity_at` field.
-   **`comfy-table`**: A library for creating beautiful and simple tables in the console, replacing `cli-table3`.
-   **`dotenv`**: For loading environment variables from a `.env` file, replacing the `dotenv` npm package.
-   **`anyhow` & `thiserror`**: For ergonomic and structured error handling throughout the application.

## 4. Data Models

We will define two primary structs in `src/types.rs` corresponding to the data shapes.

**`GitHubCopilotSeat`**: For deserializing the raw response from the GitHub API.

```rust
// In src/github_client.rs (as it's only used there)
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GitHubApiSeat {
    assignee: Assignee,
    created_at: chrono::DateTime<chrono::Utc>,
    last_activity_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Debug)]
struct Assignee {
    login: String,
}
```

**`CopilotSeat`**: The processed data model used internally by the application for display.

```rust
// In src/types.rs
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum SeatStatus {
    Active,
    Inactive,
}

#[derive(Debug)]
pub struct CopilotSeat {
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub status: SeatStatus,
}
```

## 5. Error Handling

We will use a combination of `thiserror` and `anyhow` for robust error management.

-   **`thiserror`**: In `src/error.rs`, we will define a custom `AppError` enum to represent specific, expected errors (e.g., `GitHubApiError`, `OrgNotFound`). This will allow for precise error handling in `main.rs`.
-   **`anyhow`**: Will be used in `main.rs` and other high-level functions to easily propagate errors with context, reducing boilerplate. The `main` function will return `anyhow::Result<()>`.

## 6. Testing Strategy

Testing will be a core part of the Rust project, using the built-in testing framework (`cargo test`).

-   **Unit Tests**: Each module will have a `#[cfg(test)]` section with unit tests. For example, `processor.rs` will have tests to verify the logic for determining seat status.
-   **Integration Tests**: An `tests` directory will be created for integration tests. We will use a mock HTTP server (e.g., `wiremock`) to test the `github_client.rs` module without making actual API calls, ensuring the client handles pagination and errors correctly.

## 7. Build and Execution

The project will be managed by Cargo, Rust's build tool and package manager.

-   **Execution**: `cargo run -- --organization <org_name>`
-   **Testing**: `cargo test`
-   **Production Build**: `cargo build --release` will create an optimized, single binary executable in the `target/release` directory.

This design provides a solid foundation for a reliable and efficient rewrite of the tool in Rust.