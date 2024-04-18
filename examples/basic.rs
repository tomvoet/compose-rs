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
