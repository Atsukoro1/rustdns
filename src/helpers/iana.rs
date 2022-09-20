use reqwest::get;
use crate::cache::def::RootServer;
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
        .map(|item| {
            item.to_string()
        })
        .collect::<Vec<String>>();

    // Remove first 15 lines of document info and last line containing EOF sign
    split_f.drain(0..14);
    split_f.pop().unwrap();

    vec![RootServer { qtype: todo!(), ip: todo!(), tld: todo!() }]
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