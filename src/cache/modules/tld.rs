use async_trait::async_trait;
use reqwest::get;

pub struct TLDController;

#[async_trait]
pub trait TLDT {
    fn new() -> TLDController;

    async fn load(&mut self) -> &mut TLDController;
}

#[async_trait]
impl TLDT for TLDController {
    fn new() -> TLDController {
        TLDController 
    }

    async fn load(&mut self) -> &mut TLDController {
        let response: String = get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{}", response);

        self
    }
}