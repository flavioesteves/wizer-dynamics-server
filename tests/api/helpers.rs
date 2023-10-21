use server_wizer::configuration::get_configuration;
use server_wizer::startup::Application;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to tead configuration.");
        c.application.port = 0;
        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build the application");
    let application_port = application.port();

    let address = format!("http://localhost:{}", application_port);

    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        port: application_port,
    }
}
