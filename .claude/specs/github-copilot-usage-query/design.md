# Design Document

## 1. Overview

This document provides the technical design for the GitHub Copilot Usage Query tool. It is a command-line interface (CLI) application designed to be run on-demand by a GitHub Organization Administrator. The application will be built using Node.js with TypeScript and will interact directly with the GitHub API to fetch and display real-time Copilot usage data.

## 2. Architecture

The application will follow a simple, single-executable CLI architecture. It will be compiled from TypeScript to JavaScript for execution. The architecture consists of several distinct modules, each with a specific responsibility. The data flow is unidirectional: the CLI handler receives user input, which is passed to the API client to fetch data. The raw data is then processed, formatted, and displayed to the user.

**Technology Stack:**

*   **Language:** TypeScript 5.x
*   **Runtime:** Node.js 18.x+
*   **Key Libraries:**
    *   `axios`: For making type-safe HTTP requests to the GitHub API.
    *   `yargs`: For parsing command-line arguments.
    *   `cli-table3`: For formatting the output in a clean, readable table.
    *   `dotenv`: For managing the GitHub PAT securely.

## 3. Components and Interfaces

### 3.1. CLI Handler (`src/main.ts`)

*   **Responsibility:** Serves as the entry point of the application. It parses command-line arguments and coordinates the actions of other components.
*   **Interface:**
    *   Accepts one command-line argument: `--organization <org_name>`.
    *   Reads the GitHub Personal Access Token (PAT) from an environment variable `GITHUB_PAT`.

### 3.2. GitHub API Client (`src/githubClient.ts`)

*   **Responsibility:** Manages all interactions with the GitHub API. It handles authentication, constructs requests, and retrieves data.
*   **Interface:**
    *   `createClient(token: string)`: A factory function that returns an `axios` instance pre-configured with the authentication token and base URL.
    *   `getCopilotUsage(client: AxiosInstance, orgName: string): Promise<CopilotSeat[]>`: Fetches the Copilot for Business seat usage data for the specified organization. It will handle API pagination to retrieve all records.

### 3.3. Data Processor (`src/processor.ts`)

*   **Responsibility:** Transforms the raw JSON data from the GitHub API into a structured format suitable for display.
*   **Interface:**
    *   `processUsageData(apiResponse: any[]): CopilotSeat[]`: A function that takes the raw API response array and returns a list of `CopilotSeat` objects. It will calculate the user's status (e.g., "Active" if last activity is within 30 days, "Inactive" otherwise).

### 3.4. Output Formatter (`src/formatter.ts`)

*   **Responsibility:** Displays the processed data to the user in a clear and readable format in the terminal.
*   **Interface:**
    *   `displayUsageTable(seats: CopilotSeat[]): void`: Takes the list of `CopilotSeat` objects and prints a formatted table to the console using the `cli-table3` library. It will also print the final summary counts.

## 4. Data Models

A TypeScript `interface` will be used to define the shape of a user's Copilot seat information.

**`CopilotSeat` Interface (`src/types.ts`):**

```typescript
interface CopilotSeat {
  username: string;
  createdAt: string; // ISO 8601 date string
  lastActivityAt: string | null; // ISO 8601 date string or null
  status: 'Active' | 'Inactive';
}
```

## 5. Error Handling

The application will handle errors gracefully and provide informative messages to the user.

*   **Authentication Error:** If `GITHUB_PAT` is missing, the application will exit with an error message. If the token is invalid, the `axios` client will receive a 401 error, which will be caught in `main.ts` to display a helpful message.
*   **Organization Not Found:** If the GitHub API returns a 404 status code, `axios` will throw an error. The `main.ts` module will catch this and inform the user that the organization could not be found or they lack permission.
*   **API Rate Limiting:** The `githubClient` will inspect the `AxiosError` for a 403 status and the `x-ratelimit-remaining` header to inform the user if the rate limit has been exceeded.
*   **Network Issues:** Any network-related errors from `axios` will be caught, and a general network error message will be displayed.

## 6. Testing Strategy

The testing strategy will be divided into two main parts using the Jest testing framework.

### 6.1. Unit Tests

*   **GitHub API Client:** The `axios` library will be mocked using `jest.mock` to simulate various API responses (success, 404 error, 401 error) without making actual network calls.
*   **Data Processor:** Tests will be written to ensure the processor correctly transforms various sample JSON payloads into the `CopilotSeat` interface and correctly calculates the `status`.

### 6.2. Integration Tests

*   An end-to-end test will be created that runs the compiled CLI using Node.js's `child_process` module. A fully mocked `githubClient` will be used to verify that the components work together correctly from command-line input to console output.
