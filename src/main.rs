use server_wizer::configuration::get_configuration;
use server_wizer::startup::Application;
use server_wizer::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Telemetry
    let subscriber = get_subscriber("server_wizer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Application
    let configuration = get_configuration().expect("Failed to read configuration");
    let app = Application::build(configuration).await?;
    app.run_until_stopped().await?;
    Ok(())
}
