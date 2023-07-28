use std::net::SocketAddr;

use structopt::StructOpt;

mod database;
mod handlers;
mod options;
mod routes;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let opts = options::Options::from_args();
    let users = database::Users::with_capacity(opts.users);
    let addr = SocketAddr::new(opts.ip, opts.port);
    let routes = routes::build(users);
    server::start(&addr, routes).await
}
