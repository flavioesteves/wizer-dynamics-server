use server_wizer::configuration::get_configuration;
use server_wizer::startup::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let app = Application::build(configuration).await?;
    app.run_until_stopped().await?;
    Ok(())
}
