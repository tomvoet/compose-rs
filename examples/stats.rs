use compose_rs::Compose;

fn main() {
    let compose = Compose::builder()
        .path("docker-compose.yml")
        .build()
        .unwrap();

    //let stats = compose.stats().exec().unwrap();
    //
    //for service in stats {
    //    println!("{:?}", service);
    //}

    compose.stats().stream().unwrap().for_each(|service| {
        println!("{service:?}");
    });
}
