use crate::parser::def::QuestionType;
use async_ftp::FtpStream;
use std::net::IpAddr;

pub async fn fetch_parse_rs_list() -> Vec<RootServer> {
    let mut stream = FtpStream::connect("FTP.INTERNIC.NET:21")
        .await
        .expect("Failed to connect to IANA FTP server");

    stream.login("anonymous", "anonymous").await.unwrap();

    stream.cwd("/domain")
        .await
        .unwrap();

    let remote_file = stream.simple_retr("named.cache").await.unwrap();

    let mut split_f: Vec<String> = std::str::from_utf8(&remote_file.into_inner())
        .unwrap()
        .split("\n")
        .filter(|item: &&str| {
            !item.starts_with(";")
        })
        .map(|item| {
            item.to_string()
        })
        .collect::<Vec<String>>();

    // Remove first 15 lines of document info and last line containing EOF sign
    split_f.drain(0..14);
    split_f.pop().unwrap();

    let mut final_res: Vec<RootServer> = vec![];
    for i in 0..split_f.len() - 1 {
        let item = split_f[i].split(" ")
            .filter(|el: &&str| {
                !(el.len() == 0)
            })
            .collect::<Vec<&str>>();

        final_res.push(RootServer::from_str_vec(item));
    }

    final_res
}

#[derive(Debug)]
pub struct RootServer {
    pub qtype: QuestionType,
    pub ttl: u32,

    // Either ip or domain must be defined.
    pub domain: Option<String>,
    pub ip: Option<IpAddr>,
}

pub trait RootServerT {
    /// Parse string into Root Server struct
    fn from_str_vec(src: Vec<&str>) -> RootServer;

    /// Destructure struct and return a string
    fn to_str(&self) -> String;
}

impl RootServerT for RootServer {
    fn from_str_vec(src: Vec<&str>) -> RootServer {
        println!("{:?}", src);
        RootServer { 
            qtype: (|| {
                match src[2] {
                    "NS" => QuestionType::NS,
                    "A" => QuestionType::A,
                    "AAAA" => QuestionType::AAAA,
                    _ => {
                        panic!("This question type does not exist!");
                    }
                }
            })(), 
            ttl: src[1].parse::<u32>()
                .unwrap(), 
            
            // Domains are only valid if the record type is "NS"
            domain: (|| {
                 if src[2] == "NS" {
                    Some(src[3].to_string())
                 } else {
                    None
                 }
            })(), 

            // IP address is only valid if record type is either "A" or "AAAA"
            ip: (|| {
                if src[2] == "NS" {
                    None 
                } else {
                    Some(src[3].trim().parse::<IpAddr>().unwrap())
                }
            })()
        }
    }

    fn to_str(&self) -> String {
        let mut final_str: String = String::new();

        final_str += self.ttl.to_string().as_str();
        final_str += "_";

        final_str += match self.qtype {
            QuestionType::A => "A",
            QuestionType::AAAA => "AAAA",
            QuestionType::NS => "NS",
            _ => "NS"
        };
        final_str += "_";

        if self.domain.is_some() {
            final_str += self.domain.as_ref()
                .unwrap()
                .as_str();
        } else {
            final_str += self.ip.as_ref()
                .unwrap()
                .to_string()
                .as_str();
        }

        final_str
    }
}