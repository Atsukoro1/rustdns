use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Hostname on which will the DNS server run
    pub hostname: String,

    /// Port on which will the DNS sever run
    pub port: u16,
}