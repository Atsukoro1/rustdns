use crate::parser::def::QuestionType;
use std::{net::SocketAddr, vec::IntoIter};
use redis::{
    Connection,
    Commands,
};
use tokio::sync::Mutex;
use crate::{
    CONFIG,
    helpers::iana
};

#[derive(Debug)]
pub struct RootServer {
    pub qtype: QuestionType,
    pub ttl: u32,

    // Either ip or domain must be defined.
    pub domain: Option<String>,
    pub ip: Option<SocketAddr>,
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
            redis_instance: None::<tokio::sync::Mutex<Connection>>,
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
        let redis_c = self.redis_instance.as_mut()
            .unwrap()
            .get_mut();

        match redis_c.get::<&str, Option<String>>("TLD:COM").unwrap() {
            Some(..) => {
                // Already cached
            },

            None => {
                // Not cached
                let tlds: IntoIter<String> = iana::fp_tlds()
                    .await
                    .expect("Failed to fetch TLDs")
                    .into_iter();

                tlds.for_each(|item: String| {
                    redis_c.set::<&str, String, String>(
                        format!("TLD:{}", item).as_str(), 
                        "exists".to_string()
                    ).expect("Failed to set TLD");
                });
            }
        }

        Ok(())
    }
}