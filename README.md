# Pantheon

`Pantheon` is a versatile web template built upon the [axum](https://github.com/tokio-rs/axum) framework.

## Overview

Pantheon is a web application template designed to provide a robust foundation for developing web applications using Rust. The project structure adheres to specific conventions to ensure a standardized and scalable development process.

## Usage

1. Clone the repository.
2. Customize configurations in the `.env` and `config.toml` files as required.
3. Perform Database Migration.
    ```bash
    # Ensure that `sqlx-cli` is installed. You can install it using the following command:
    cargo install sqlx-cli

    # Ensure the database connection information is properly configured in the `.env` file.
    DATABASE_URL=postgres://username:password@host/database

    # Create the database at DATABASE_URL:
    sqlx database create

    # Execute the data migration command to set up the database tables:
    sqlx migrate run
    ```
4. Run the application with `cargo run`.
5. Access the application through specified endpoints and utilize the provided API functionalities.

## Minimum supported Rust version

pantheon's MSRV is 1.70.

## License

Licensed under the [MIT License](LICENSE).