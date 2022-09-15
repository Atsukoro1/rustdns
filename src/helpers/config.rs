use configparser::ini::Ini;

pub struct Config {
    pub hostname: String,
    pub port: u16
}

/// Load config from config.conf file located in root directory
/// Returns config struct
pub fn load_config() -> Config {
    let mut config = Ini::new();
    config.load(std::path::Path::new("./config.conf"))
        .expect(
            "Malformed config, try copying config from the
            repo's readme or create a new issue on github."
        );

    Config { 
        hostname: config.get("host", "hostname").unwrap(),
        port: config.get("host", "port")
            .unwrap()
            .parse::<u16>()
            .unwrap()
    }
} 