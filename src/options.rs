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

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use clap::Parser;

    use super::*;

    const APP_BINARY_NAME: &str = "app";

    #[test]
    fn parse_no_arguments() {
        let cmd = &[APP_BINARY_NAME];

        let options = Options::try_parse_from(cmd).expect("Failed to parse empty arguments");

        assert_eq!(options.ip, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(options.port, 8080);
        assert_eq!(options.users, 100);
    }
}
