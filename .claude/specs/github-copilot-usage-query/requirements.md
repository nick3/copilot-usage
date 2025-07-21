# Requirements Document

## Introduction

This document outlines the requirements for a program that provides real-time querying of GitHub Copilot Business usage for all members within a specific GitHub Organization. This tool will help administrators monitor license allocation, identify inactive users, and manage their Copilot subscription effectively.

## Requirements

### Requirement 1: Authenticate with GitHub

**User Story:** As an Organization Administrator, I want to securely authenticate with the GitHub API, so that I can access Copilot usage data for my organization.

#### Acceptance Criteria

1.  **WHEN** the program starts, **THEN** the system **SHALL** prompt for a GitHub Personal Access Token (PAT) with the necessary permissions (`manage_billing:copilot`).
2.  **IF** an invalid token is provided, **THEN** the system **SHALL** display an authentication error message and terminate.
3.  **IF** the provided token is valid, **THEN** the system **SHALL** confirm successful authentication.

### Requirement 2: Fetch Copilot Usage Data

**User Story:** As an Organization Administrator, I want to fetch the GitHub Copilot usage data for all members of my organization, so that I can see who has an active seat.

#### Acceptance Criteria

1.  **WHEN** I run the program with a valid organization name, **THEN** the system **SHALL** make an API request to the GitHub endpoint for Copilot usage.
2.  **IF** the organization does not exist or the authenticated user does not have permission, **THEN** the system **SHALL** display an appropriate error message.
3.  **WHEN** the data is fetched successfully, **THEN** the system **SHALL** process the list of all members with a Copilot Business seat.

### Requirement 3: Display Usage Information

**User Story:** As an Organization Administrator, I want to view a clear, real-time summary of Copilot usage, so that I can quickly understand the current status.

#### Acceptance Criteria

1.  **WHEN** the usage data is processed, **THEN** the system **SHALL** display a list of all members with an assigned Copilot seat.
2.  **FOR EACH** member, the system **SHALL** display the following information:
    *   GitHub username (handle)
    *   Last activity date
    *   Seat creation date
    *   Current status (e.g., Active, Inactive)
3.  **WHEN** the program finishes, **THEN** the system **SHALL** display a summary count of total active seats and total assigned seats.

### Requirement 4: Real-time Querying

**User Story:** As an Organization Administrator, I want the data to be queried in real-time, so that I always have the most up-to-date information.

#### Acceptance Criteria

1.  **WHEN** I execute the program, **THEN** the system **SHALL** always fetch the latest data from the GitHub API and not use a cache.
