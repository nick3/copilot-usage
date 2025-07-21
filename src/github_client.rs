//! Handles all communication with the GitHub API.

use crate::error::AppError;
use reqwest::Client;
use serde::Deserialize;

/// Represents a single seat as returned by the GitHub API.
#[derive(Deserialize, Debug)]
pub struct GitHubApiSeat {
    /// The user or team assigned to the seat.
    pub assignee: Assignee,
    /// The timestamp when the seat was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The timestamp of the user's last activity, if available.
    pub last_activity_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Represents the assignee of a Copilot seat.
#[derive(Deserialize, Debug)]
pub struct Assignee {
    /// The GitHub login of the assignee.
    pub login: String,
}

/// Internal struct to represent the nested structure of the API response.
#[derive(Deserialize, Debug)]
struct SeatsApiResponse {
    seats: Vec<GitHubApiSeat>,
}

/// Creates and configures a new `reqwest` client.
pub fn create_client() -> Result<Client, AppError> {
    Client::builder()
        .user_agent("GithubCopilotUsages-Rust-CLI")
        .build()
        .map_err(AppError::GitHubApiError)
}

/// Fetches all Copilot usage seats for a given organization.
///
/// This function handles pagination to retrieve all seats.
///
/// # Arguments
///
/// * `client` - The `reqwest` client instance.
/// * `org_name` - The name of the GitHub organization.
/// * `token` - The GitHub Personal Access Token.
pub async fn get_copilot_usage(
    client: &Client,
    org_name: &str,
    token: &str,
) -> Result<Vec<GitHubApiSeat>, AppError> {
    if token.is_empty() {
        return Err(AppError::MissingToken);
    }

    let mut all_seats = Vec::new();
    let mut page = 1;
    let per_page = 100;

    loop {
        let url = format!("https://api.github.com/orgs/{}/copilot/billing/seats", org_name);
        
        let response = client
            .get(&url)
            .bearer_auth(token)
            .header("Accept", "application/vnd.github.v3+json")
            .query(&[("per_page", per_page.to_string()), ("page", page.to_string())])
            .send()
            .await;

        let res = match response {
            Ok(res) => res,
            Err(e) => return Err(AppError::GitHubApiError(e)),
        };

        if res.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(AppError::OrgNotFound(org_name.to_string()));
        }

        if !res.status().is_success() {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_else(|_| "Unknown API error".to_string());
            return Err(AppError::Unknown(format!("API request failed with status {}: {}", status, err_text)));
        }
        
        let api_response = res.json::<SeatsApiResponse>().await?;
        
        if api_response.seats.is_empty() {
            break; // No more seats, exit the loop
        }
        
        all_seats.extend(api_response.seats);
        page += 1;
    }

    Ok(all_seats)
}
