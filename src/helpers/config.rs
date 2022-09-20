use configparser::ini::Ini;

pub enum LoggingType {
    TERMINAL,
    FILE
}

pub struct Config {
    pub hostname: String,
    pub port: u16,
    pub redis_addr: String,
    pub log_type: Option<LoggingType>,
    pub on: bool,
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
        redis_addr: config.get("cache", "redis_addr").unwrap(),
        hostname: config.get("host", "hostname").unwrap(),
        port: config.get("host", "port")
            .unwrap()
            .parse::<u16>()
            .unwrap(),
        on: config.get("logging", "on")
            .unwrap()
            .eq("yes")
            .then(|| true)
            .or_else(|| enum_primitive::Option::Some(false))
            .unwrap(),
        log_type: enum_primitive::Option::Some((|| {
            let lt = config.get("logging", "type")
                .unwrap();

            if lt == "file" {
                LoggingType::FILE
            } else {
                LoggingType::TERMINAL
            }
        })())
    }
} 