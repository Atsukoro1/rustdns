use crate::{CONFIG, LOGGER, cache::modules::rootserver::RootServerT, parser::def::QuestionType};
use tokio::sync::Mutex;
use std::vec::IntoIter;
use super::modules::{
    tld::fp_tlds, 
    rootserver::fetch_parse_rs_list, 
    rootserver::RootServer
};
use redis::{
    Connection,
    Commands,
};
use slog::info;

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
    /// Here are the formats resources are cached in ->
    /// 
    /// 1. Top level domains -> TLD:<domain>
    /// 
    /// 2. Root servers -> ROOTS:<qtype> as a key and 
    /// list of root servers separated by "\r" character in following 
    /// format tld_ip/domain as value
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
        let connection = redis::Client::open(&*CONFIG.cache.hostname)
            .unwrap()
            .get_connection();

        match connection {
            Ok(..) => {
                return Ok(self.redis_instance = Some(
                    Mutex::from(
                        connection.expect("Failed to start redis cache")
                    )
                ));
            },

            Err(e) => {
                return Result::Err::<(), String>(
                    e.to_string()
                );
            }
        }
    }

    async fn load_resources(&mut self) -> Result<(), String> {
        let redis_c = self.redis_instance.as_mut()
            .unwrap()
            .get_mut();

        match redis_c.get::<&str, Option<String>>("TLD:COM").unwrap() {
            Some(..) => {
                // Already cached
                info!(LOGGER, "TLDs are already cached!");
            },

            None => {
                // Not cached
                info!(LOGGER, "Caching TLDs...");
                let tlds: IntoIter<String> = fp_tlds()
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

        match redis_c.get::<&str, Option<String>>("ROOTS:NS").unwrap() {
            Some(..) => {
                // Already cached
                info!(LOGGER, "Root servers are already cached!");
            },

            None => {
                // Not cached
                info!(LOGGER, "Caching root servers!");
                let root_servers: IntoIter<RootServer> = fetch_parse_rs_list()
                    .await
                    .into_iter();

                root_servers.for_each(|item: RootServer| {
                    redis_c.append::<String, String, String>(
                        match item.qtype {
                            QuestionType::NS => "ROOTS:NS",
                            QuestionType::A => "ROOTS:A",
                            QuestionType::AAAA  => "ROOTS:AAAA",
                            _ => "NS"
                        }.to_string(), 
                        item.to_str()
                    )
                    .is_err()
                    .then(|| {
                        return Result::Err::<(), String>(
                            String::from("Failed to cache one of the root servers!")
                        );
                    });
                });
            }
        }

        Ok(())
    }
}