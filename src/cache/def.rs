use redis::Connection;
use crate::CONFIG;

pub struct CacheManager {
    pub redis_instance: Option<Connection>
}

pub trait CMTrait {
    /// Creates a new instance of Cache manager
    fn new() -> CacheManager;

    /// Estabilish connection within the redis instance and the
    /// cache manager
    /// 
    /// Can return error in String format
    fn connect(&mut self) -> Result<(), String>;

    /// Load resources from IANA
    /// 
    /// Helpers will fetch following resources and return them here to cache
    /// https://www.internic.net/domain/named.root
    /// https://www.internic.net/domain/root.zone
    /// https://data.iana.org/TLD/tlds-alpha-by-domain.txt
    /// 
    /// Can return error in String format
    fn load_resources(&mut self) -> Result<(), String>;
}

impl CMTrait for CacheManager {
    fn new() -> CacheManager {
        CacheManager { redis_instance: None }
    }

    fn connect(&mut self) -> Result<(), String> {
        let connection = redis::Client::open(&*CONFIG.redis_addr)
            .unwrap()
            .get_connection();

        connection
            .is_err()
            .then(|| {
                return Err::<(), String>(String::from(
                    "Failed to estabilish connection with Redis instance"
                ));
            });

        self.redis_instance = Some(connection.unwrap());

        Ok(())
    }

    fn load_resources(&mut self) -> Result<(), String> {
        Ok(())
    }
}