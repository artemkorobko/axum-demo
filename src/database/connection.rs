use std::ops::Deref;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use super::repository;

#[derive(Clone)]
pub struct ConnectionPool(Pool<SqliteConnectionManager>);

impl Deref for ConnectionPool {
    type Target = Pool<SqliteConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ConnectionPool {
    pub fn new(
        manager: r2d2_sqlite::SqliteConnectionManager,
        size: u32,
    ) -> Result<Self, r2d2::Error> {
        r2d2::Pool::builder()
            .max_size(size)
            .build(manager)
            .map(ConnectionPool)
    }

    pub fn connection(&self) -> Result<Connection, r2d2::Error> {
        self.0.get().map(Connection)
    }
}

pub struct Connection(PooledConnection<SqliteConnectionManager>);

impl From<PooledConnection<SqliteConnectionManager>> for Connection {
    fn from(value: PooledConnection<SqliteConnectionManager>) -> Self {
        Self(value)
    }
}

impl Deref for Connection {
    type Target = PooledConnection<SqliteConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Connection {
    pub fn login_repository(self) -> repository::Login {
        repository::Login::from(self)
    }
}
