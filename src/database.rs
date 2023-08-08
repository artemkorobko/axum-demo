pub mod connection;
pub mod repository;
pub mod users;

pub use connection::{Connection, ConnectionPool};
pub use users::{UserEntry, Users};

refinery::embed_migrations!("migrations");

#[cfg(test)]
pub mod tests {
    use super::*;

    lazy_static::lazy_static! {
        pub static ref POOL: ConnectionPool = {
            let manager = r2d2_sqlite::SqliteConnectionManager::memory();
            let pool = ConnectionPool::new(manager, 1).expect("Failed to create connection pool for memory SQLite database");
            let mut conn = pool.get().expect("Failed to get database connection from the pool");
            migrations::runner().run(&mut *conn).expect("Failed to run database migration script");
            pool
        };
    }
}
