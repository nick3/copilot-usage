//! Contains the business logic for processing raw API data.

use crate::github_client::GitHubApiSeat;
use crate::types::{CopilotSeat, SeatStatus};
use chrono::{Duration, Utc};

/// Processes a vector of raw `GitHubApiSeat` data into a vector of `CopilotSeat`.
///
/// This function calculates the activity status of each seat based on its
/// `last_activity_at` timestamp. A seat is considered 'Active' if the user
/// has been active within the last 30 days.
///
/// # Arguments
///
/// * `api_response` - A vector of `GitHubApiSeat` structs from the API client.
pub fn process_usage_data(api_response: Vec<GitHubApiSeat>) -> Vec<CopilotSeat> {
    let thirty_days_ago = Utc::now() - Duration::days(30);

    api_response
        .into_iter()
        .map(|seat| {
            let status = match seat.last_activity_at {
                Some(last_activity) if last_activity > thirty_days_ago => SeatStatus::Active,
                _ => SeatStatus::Inactive,
            };

            CopilotSeat {
                username: seat.assignee.login,
                created_at: seat.created_at,
                last_activity_at: seat.last_activity_at,
                status,
            }
        })
        .collect()
}
