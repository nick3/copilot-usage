//! Responsible for displaying the processed data to the user in the console.

use crate::types::{CopilotSeat, SeatStatus};
use comfy_table::{presets::UTF8_FULL, Cell, ContentArrangement, Table};

/// Displays the provided Copilot usage data in a formatted table.
///
/// Also prints a summary of total and active seats.
///
/// # Arguments
///
/// * `seats` - A slice of `CopilotSeat` structs to display.
pub fn display_usage_table(seats: &[CopilotSeat]) {
    if seats.is_empty() {
        println!("No Copilot seats found for this organization.");
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Username", "Status", "Last Activity", "Seat Created"]);

    let mut active_seats = 0;
    for seat in seats {
        if seat.status == SeatStatus::Active {
            active_seats += 1;
        }

        let last_activity_str = seat
            .last_activity_at
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "N/A".to_string());
        
        let created_at_str = seat.created_at.format("%Y-%m-%d %H:%M:%S").to_string();

        table.add_row(vec![
            Cell::new(&seat.username),
            Cell::new(&seat.status.to_string()),
            Cell::new(&last_activity_str),
            Cell::new(&created_at_str),
        ]);
    }

    println!("{table}");
    println!("\nSummary:");
    println!("- Total Assigned Seats: {}", seats.len());
    println!("- Total Active Seats:   {}", active_seats);
}
