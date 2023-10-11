use crate::configuration::get_configuration;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: u16,
}

impl Config {
    pub fn init() -> Config {
        let configuration = get_configuration().expect("Failed to read configuration");

        let jwt_secret = configuration.jwt.secret;
        let jwt_expires_in = configuration.jwt.expired_in;
        let jwt_maxage = configuration.jwt.max_age;

        Config {
            jwt_secret,
            jwt_expires_in,
            jwt_maxage,
        }
    }
}
