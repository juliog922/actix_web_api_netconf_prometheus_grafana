# Rust Application with Actix Web and Prometheus

## Project Structure

The project structure is organized as follows:

- `ncclient_3/`: Rust application source code.
  - `dockerfile`: Dockerfile for building the Rust application.
  - `src/`: Source directory containing Rust code.
    - `main.rs`: Main entry point of the Actix Web application.
    - `models.rs`: Definitions of data models.
    - `opt_utils.rs`: Utilities for handling Prometheus metrics.
    - `routes/`: Module containing route handlers.
    - `utils/`: Utility functions.
- `prometheus.yaml`: Prometheus configuration file.
- `docker-compose.yml`: Docker Compose configuration file.

## Setup and Configuration

1. **Build and Run the Application:**

   ```bash
   docker-compose up --build

## Application Details

`main.rs`

The main entry point of the Actix Web application. 
Initializes HTTP server, registers routes, and configures Prometheus metrics.

## Prometheus Integration

Metrics for the application are collected using Prometheus, with configuration defined in prometheus.yaml.

## Docker Compose Configuration

The docker-compose.yml file defines services for Rust application, Prometheus, and Grafana. 
It sets up networking between containers and volumes for data persistence.

## Routes

- **GET** `/get_json/{host}`: Retrieves JSON data from a network device specified by {host}.
- **POST** `/add_host`: Adds a new host with parameters (host, port, user, password) to the application.