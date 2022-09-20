use std::net::SocketAddr;

use reqwest::get;
use crate::{cache::def::RootServer, parser::def::QuestionType};
use async_ftp::FtpStream;

/// Returns sorted vector of Root servers
pub async fn fp_root_servers() -> Vec<RootServer> {
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

        match item[3].parse::<SocketAddr>() {
            // Record does contain valid IP address and is probably of type A or AAAA
            Ok(addr) => {
                final_res.push(
                    RootServer {
                        ip: enum_primitive::Option::Some(addr),
                        qtype: (|| {
                            match item[2] {
                                "NS" => QuestionType::NS,
                                "A" => QuestionType::A,
                                "AAAA" => QuestionType::AAAA,
                                _ => {
                                    panic!("This question type does not exist!");
                                }
                            }
                        })(),
                        ttl: item[1].parse::<u32>()
                            .unwrap(),
                        domain: None
            }
        );
            },

            // This is Nameserver record
            Err(..) => {
                final_res.push(
                    RootServer { 
                        qtype: (|| {
                            match item[2] {
                                "NS" => QuestionType::NS,
                                "A" => QuestionType::A,
                                "AAAA" => QuestionType::AAAA,
                                _ => {
                                    panic!("This question type does not exist!");
                                }
                            }
                        })(), 
                        ttl: item[1].parse::<u32>()
                            .unwrap(), 
                        domain: Some(item[3].to_string()), 
                        ip: None
                    }
                )
            }
        }
    }

    final_res
}

/// Returns sorted Vector of TLDs
pub async fn fp_tlds() -> Result<Vec<String>, reqwest::Error> {
    let text_res: String = get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt")
        .await?
        .text()
        .await?;

    let mut split_r: Vec<String> = text_res.split("\n")
        .into_iter()
        .map(|item: &str| {
            item.to_string()
        })
        .collect::<Vec<String>>();

    // Remove the version and date info and the last EOF byte
    split_r.remove(0);
    split_r.remove(split_r.len() - 1);

    /*
        Sort the list alphabetical, so binary search can be used 
        with this vector
    */
    split_r.sort_by(|st, nd| {
        return st.partial_cmp(nd).unwrap()
    });

    Ok(
        split_r
    )
}