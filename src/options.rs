use std::net::IpAddr;

/// A basic Axum server example
#[derive(structopt::StructOpt, Debug)]
pub struct Options {
    /// Listener's IP address
    #[structopt(short, long, default_value = "0.0.0.0")]
    pub ip: IpAddr,
    /// Listener's port
    #[structopt(short, long, default_value = "8080")]
    pub port: u16,
    /// Users database capacity
    #[structopt(short, long, default_value = "100")]
    pub users: usize,
}
