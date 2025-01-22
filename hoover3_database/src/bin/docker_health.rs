use hoover3_database::tracing::init_tracing;

#[tokio::main]
async fn main() -> Result<(), String> {
    init_tracing();
    println!(
        "DOCKER HEALTH STATUS\n{:#?}\nDONE",
        hoover3_database::client_query::docker_health::get_container_status(()).await?
    );
    Ok(())
}
