# Task Management Web Application

## Overview

This project is a simplified task management web application SaaS developed in Rust using the Actix web framework. The application allows multiple users to create, update, delete, and list their tasks. Each task includes a title, description, due date, and status. The application aims to demonstrate proficiency in Rust, adherence to functional programming principles, and the ability to design, implement, and test a scalable, fault-tolerant, and responsive system.

## Architecture

### Components

1. **Backend**:
    - **Rust & Actix Web**: The backend is implemented using Rust and the Actix web framework.
    - **In-Memory Database**: User and task data are stored in an in-memory database using a `HashMap`.
    - **Concurrency**: Rust's concurrency features are utilized to manage state efficiently.
    - **Error Handling**: Robust error handling ensures the system is fault-tolerant.

2. **Data Structures**:
    - **User**: Each user has a unique ID and a collection of tasks.
    - **Task**: Each task has a unique ID, title, description, due date (as `chrono::NaiveDate`), and status.
    - **Status**: An enumeration representing the status of a task, with the following variants:
      - `ToDo`
      - `InProgress`
      - `Done`

3. **Functional Programming Principles**:
    - **Immutability**: Wherever possible, data is immutable to avoid side effects.
    - **Pure Functions**: Functions are designed to be pure, receiving input and returning output without altering state directly.
    - **Concurrency**: Mutexes and locks are used to manage shared state safely.

## Design Decisions

1. **Web Framework**: Actix was chosen for its performance, robustness, and ecosystem.
2. **Date Handling**: The `chrono` crate is used for robust date handling.
3. **In-Memory Storage**: For simplicity, an in-memory `HashMap` is used for storing user and task data. This can be replaced with a persistent database in a real-world scenario.
4. **Functional Programming**: Emphasis on immutability and pure functions to enhance reliability and maintainability.

## Setup and Usage

### Prerequisites

- **Rust**: Ensure Rust is installed on your system. You can install it from [here](https://www.rust-lang.org/tools/install).
- **Cargo**: Rust's package manager, typically included with the Rust installation.

### Installation

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/Rahul721999/task-manager_makerstudio.git
    cd /task-manager_makerstudio
    ```

2. **Install Dependencies**:
    ```sh
    cargo build
    ```

3. **Run the Server**:
    ```sh
    cargo run
    ```

### API Endpoints

- **Create a User**:
    - **POST** `/users/create`
    - **Request Body**: `{ "name": "User Name" }`
    - **Response**: `200 OK`, `{ "<UUID>" }`

- **Delete a User**:
    - **DELETE** `/users/delete`
    - **Request Body**: `{ "id": "<UUID>"}`
    - **Response**: `200 OK`, `"UserID: <UUID> deleted"`

- **Create a Task**:
    - **POST** `/users/{userId}/tasks/create`
    - **Request Body**: `{ 
            "title": "Task Title", 
            "description": "Task Description", 
            "due_date": "YYYY-MM-DD" 
        }`
    - **Response**: `200 OK`, `{ "<UUID>" }`

- **List a Task**:
    - **GET** `/users/{userId}/tasks/list`
    - **Response**: `200 OK`, `[{ "task": { "id": "<UUID>", "title": "Task Title", "description": "Task Description", "due_date": "YYYY-MM-DD", "status": "ToDo" } }]`

- **Update a Task**:
    - **PUT** `/users/{userId}/tasks/update`
    - **Request Body**: `{ "id": "<UUID>", "status": "InProgress" / "Done" }`
    - **Response**: `200 OK`, `"<UUID>"`

- **Delete a Task**:
    - **DELETE** `/users/{userId}/tasks`
    - **Request Body**: `{ "id": "<UUID>"}`
    - **Response**: `200 OK`, `"<UUID>"`

### Error Handling

- **500 Internal Server Error**: Returned when there is an issue with the server, such as failing to acquire a lock on the state data.

### Logging

- The application uses the `log` crate for logging.
- Logs are output to the console to help with debugging and monitoring.

## Testing

- **Unit Tests**: Tests are written to ensure the correctness of individual functions.
- **Integration Tests**: Tests are written to ensure that different parts of the application work together correctly.
- **Running Tests**:
    ```sh
    cargo test
    ```

## Conclusion

This project is to showcase my knowledge of Rust and the Actix web framework to build a scalable and responsive web application following functional programming principles.
