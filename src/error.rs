//! Defines the custom error types used throughout the application.

use thiserror::Error;

/// The primary error type for this application.
#[derive(Error, Debug)]
pub enum AppError {
    /// An error occurred while making a request to the GitHub API.
    #[error("GitHub API error: {0}")]
    GitHubApiError(#[from] reqwest::Error),

    /// The GitHub Personal Access Token was not found in the environment.
    #[error("Missing GitHub Personal Access Token (GITHUB_PAT). Please set it in your environment or a .env file.")]
    MissingToken,

    /// The specified organization could not be found or the user lacks permissions.
    #[error("Organization '{0}' not found or you lack permission to access its billing information.")]
    OrgNotFound(String),

    /// An unexpected or otherwise uncategorized error occurred.
    #[error("An unknown error has occurred: {0}")]
    Unknown(String),
}
