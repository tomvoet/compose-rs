# compose-rs

**compose-rs** is a Rust library designed to manage Docker Compose environments programmatically. It provides a straightforward API for executing common Docker Compose commands directly from Rust code.

## Features

- **Easy Setup**: Quickly configure your Docker Compose path and start managing containers.
- **Command Execution**: Support for basic Docker Compose commands like `up`, `down`, `ps`, `stats`, `scale`, and `start`.
- **Stream Stats**: Stream statistics of services in real-time.

## Installation

Add to your `Cargo.toml`:

```diff
[dependencies]
+ compose-rs = "0.0.4"
```

## Quick Start

This example demonstrates how to bring up a Docker Compose environment and monitor the stats of running services in real-time.

```rust
use compose_rs::{Compose, ComposeCommand};

fn main() {
    let compose = Compose::builder()
        .path("docker-compose.yml")
        .build()
        .unwrap();

    // Execute the `up` command to start services defined in the Docker Compose file
    if let Err(e) = compose.up().exec() {
        eprintln!("Error starting services: {}", e);
    }

    // Stream stats and print them in real-time for each service
    compose
        .stats()
        .stream()
        .unwrap()
        .into_iter()
        .for_each(|service| {
            println!("{:?}", service);
        });

    // After monitoring, bring down the services
    if let Err(e) = compose.down().exec() {
        println!("Error stopping services: {}", e);
    }
}
```

## Documentation

For detailed API documentation and advanced usage, please refer to the generated documentation.

## Contributing

Contributions are welcome! Please feel free to contribute by opening issues or submitting pull requests.

## License

**compose-rs** is licensed under the MIT license. See LICENSE for details.