use compose_rs::{Compose, ComposeCommand};

fn main() {
    let compose = Compose::builder()
        .path("docker-compose.yml")
        .build()
        .unwrap();

    if let Err(e) = compose.up().exec() {
        println!("Error: {e}");
    }

    match compose.ps().exec() {
        Ok(ps) => {
            for service in ps {
                println!("{service:?}");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }

    if let Err(e) = compose.scale().service(2, "rqlite").exec() {
        println!("Error: {e}");
    }

    match compose.ps().exec() {
        Ok(ps) => {
            for service in ps {
                println!("{service:?}");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }

    if let Err(e) = compose.down().exec() {
        println!("Error: {e}");
    }
}
