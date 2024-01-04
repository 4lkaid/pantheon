# Pantheon

`Pantheon` is a versatile web template built upon the [axum](https://github.com/tokio-rs/axum) framework.

## Overview

Pantheon is a web application template designed to provide a robust foundation for developing web applications using Rust. The project structure adheres to specific conventions to ensure a standardized and scalable development process.

## Usage

1. Clone the repository.
2. Customize configurations in the `.env` and `config.toml` files as required.
3. Perform Database Migration.
    - Ensure that `sqlx-cli` is installed. You can install it using the following command:
        ```bash
        cargo install sqlx-cli
        ```
    - Ensure the database connection information is properly configured in the `.env` file.
        ```bash
        DATABASE_URL=postgres://username:password@host/database
        ```
    - Execute the data migration command to set up the database tables:
        ```bash
        sqlx migrate run
        ```
4. Run the application with `cargo run`.
5. Access the application through specified endpoints and utilize the provided API functionalities.

## License

Licensed under the [MIT License](LICENSE).