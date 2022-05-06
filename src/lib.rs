use std::env::var;
use std::net::SocketAddr;

pub struct Config {
    server_port: String,
    server_ip: String,
    pub db_location: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            server_port: var("SSS_SERVER_PORT").unwrap_or("3000".to_string()),
            server_ip: var("SSS_SERVER_IP").unwrap_or("0.0.0.0".to_string()),
            db_location: var("SSS_DB_LOCATION").unwrap_or("/tmp/sss_db".to_string()),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", &self.server_ip, &self.server_port)
            .parse()
            .expect("Unable to parse address")
    }
}
