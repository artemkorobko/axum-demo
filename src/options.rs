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
    fn parse_ip_argument(#[case] key: &str, #[case] value: &str, #[case] expected: Ipv4Addr) {
        let cmd = &[APP_BINARY_NAME, key, value];

        let options = Options::try_parse_from(cmd).expect("Failed to parse ip argument");

        assert_eq!(options.ip, expected);
    }

    #[rstest]
    #[case("-i", "invalid", "invalid value 'invalid' for '--ip <IP>'")]
    #[case(
        "--ip",
        "255.x.255.255",
        "invalid value '255.x.255.255' for '--ip <IP>'"
    )]
    fn return_invalid_ip_argument_error(#[case] key: &str, #[case] value: &str, #[case] msg: &str) {
        let cmd = &[APP_BINARY_NAME, key, value];

        let err = Options::try_parse_from(cmd)
            .err()
            .expect("Invalid address should not be parsed");

        assert!(err.to_string().contains(msg));
    }

    #[rstest]
    #[case("-p", "3000", 3000)]
    #[case("--port", "4000", 4000)]
    fn parse_port_argument(#[case] key: &str, #[case] value: &str, #[case] expected: u16) {
        let cmd = &[APP_BINARY_NAME, key, value];

        let options = Options::try_parse_from(cmd).expect("Failed to parse port argument");

        assert_eq!(options.port, expected);
    }

    #[rstest]
    #[case("-p", "invalid", "invalid value 'invalid' for '--port <PORT>'")]
    #[case("--port", "-10", "unexpected argument '-1' found")]
    fn return_invalid_port_argument_error(
        #[case] key: &str,
        #[case] value: &str,
        #[case] msg: &str,
    ) {
        let cmd = &[APP_BINARY_NAME, key, value];

        let err = Options::try_parse_from(cmd)
            .err()
            .expect("Invalid post should not be parsed");

        assert!(err.to_string().contains(msg));
    }

    #[rstest]
    #[case("-u", "10", 10)]
    #[case("--users", "20", 20)]
    fn parse_users_argument(#[case] key: &str, #[case] value: &str, #[case] expected: usize) {
        let cmd = &[APP_BINARY_NAME, key, value];

        let options = Options::try_parse_from(cmd).expect("Failed to parse users argument");

        assert_eq!(options.users, expected);
    }
}
