use crate::{
    parser::def::QuestionType,
    helpers::iana, 
};
use std::net::SocketAddr;
use tokio::sync::Mutex;
use redis::{
    Connection, 
    Commands
};
use crate::CONFIG;

#[derive(Debug)]
pub struct RootServer {
    pub qtype: QuestionType,
    pub ip: SocketAddr,
    pub tld: u32
}

pub struct CacheManager {
    pub redis_instance: Option<Mutex<Connection>>,
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
    /// Resources are only loaded if the result is not already cached
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

        self.redis_instance = Some(Mutex::from(connection.unwrap()));

        Ok(())
    }

    async fn load_resources(&mut self) -> Result<(), String> {
        // let mut redis_c = self.redis_instance.as_ref().unwrap().lock().await;
        // redis_c.set::<&str, &str, String>("fdsfd", "fdf").expect("Failed XDDD");
        println!("{:?}", iana::fp_root_servers().await);
        Ok(())
    }
}