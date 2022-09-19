use redis::Connection;
use crate::CONFIG;
use super::modules::{
    tld::{TLDController, TLDT}, 
    root::RootController, 
    zone::ZoneController
};

pub struct CacheManager {
    pub redis_instance: Option<Connection>,
    
    // Controllers
    pub tld_controller: TLDController,
    pub root_controller: RootController,
    pub zone_controller: ZoneController
}

#[async_trait::async_trait]
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
    async fn load_resources(&mut self) -> Result<(), String>;
}

#[async_trait::async_trait]
impl CMTrait for CacheManager {
    fn new() -> CacheManager {
        CacheManager { 
            redis_instance: None,
            tld_controller: TLDController,
            root_controller: RootController,
            zone_controller: ZoneController
        }
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

    async fn load_resources(&mut self) -> Result<(), String> {
        self.tld_controller.load().await;
        Ok(())
    }
}