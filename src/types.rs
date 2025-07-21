//! Defines the core data structures for representing Copilot usage information.

use chrono::{DateTime, Utc};

/// Represents the activity status of a Copilot seat.
#[derive(Debug, PartialEq)]
pub enum SeatStatus {
    /// The user has been active in the last 30 days.
    Active,
    /// The user has not been active in the last 30 days.
    Inactive,
}

impl std::fmt::Display for SeatStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeatStatus::Active => write!(f, "Active"),
            SeatStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

/// Represents a single GitHub Copilot seat with processed usage details.
#[derive(Debug)]
pub struct CopilotSeat {
    /// The GitHub username of the assignee.
    pub username: String,
    /// The timestamp when the seat was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp of the user's last activity, if available.
    pub last_activity_at: Option<DateTime<Utc>>,
    /// The calculated status of the seat (Active or Inactive).
    pub status: SeatStatus,
}
