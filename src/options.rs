use std::net::IpAddr;

/// A basic Axum server example
#[derive(clap::Parser, Debug)]
pub struct Options {
    /// Listener's IP address
    #[arg(short, long, default_value = "0.0.0.0")]
    pub ip: IpAddr,
    /// Listener's port
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
    /// Users database capacity
    #[arg(short, long, default_value = "100")]
    pub users: usize,
}
