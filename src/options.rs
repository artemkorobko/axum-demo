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
    use rstest::rstest;

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

    #[rstest]
    #[case("-i", "127.0.0.1", Ipv4Addr::new(127, 0, 0, 1))]
    #[case("--ip", "192.168.0.1", Ipv4Addr::new(192, 168, 0, 1))]
    fn parse_ip_address(#[case] key: &str, #[case] value: &str, #[case] expected: Ipv4Addr) {
        let cmd = &[APP_BINARY_NAME, key, value];

        let options = Options::try_parse_from(cmd).expect("Failed to parse ip address argument");

        assert_eq!(options.ip, expected);
    }
}
