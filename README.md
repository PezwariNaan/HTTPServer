# HTTP Server with Thread Pool

This repository contains a simple multi-threaded HTTP server implemented in Rust. It uses a custom thread pool to handle incoming client connections concurrently, allowing it to serve multiple requests efficiently. The server listens for HTTP requests on a configurable port and logs details of each request to a specified file.

## Project Structure

- **`main.rs`**: Entry point for the server, sets up the listener, thread pool, and begins handling incoming requests.
- **`thread_pool.rs`**: Contains the `ThreadPool` and `Worker` implementations to manage concurrent tasks using Rust’s threading and messaging capabilities.
- **`index.html`**: Default webpage served on successful requests.
- **`404.html`**: Page served when a request cannot be resolved.

## Key Features

- **Thread Pool**: Uses a thread pool to efficiently handle multiple requests concurrently.
- **Logging**: Logs incoming request details (client IP and request content) to a specified log file.
- **Auto-Creation of Log Files and Directories**: Ensures the required log directories and files exist before server operations begin.
- **Customizable**: Easily extend the server to handle different routes or more complex logging needs.