// use rusqlite::Connection;

use anyhow::Context;
use uuid::Uuid;

use crate::database::connection::Connection;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LoginStatus {
    Active,
    Disabled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LoginEntry {
    pub id: usize,
    pub hash: Uuid,
    pub email: String,
    pub password: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub status: LoginStatus,
}

pub struct Login {
    connection: Connection,
}

impl From<Connection> for Login {
    fn from(value: Connection) -> Self {
        Self { connection: value }
    }
}

impl Login {
    pub fn create(&self, email: &str, password: &str) -> anyhow::Result<usize> {
        let hash = Uuid::new_v4().as_simple().to_string();
        self.connection
            .execute(
                "INSERT INTO login (hash, email, password) VALUES (:hash, :email, :password)",
                &[
                    (":hash", hash.as_str()),
                    (":email", email),
                    (":password", password),
                ],
            )
            .with_context(|| format!("Failed to create user with email {email}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::tests::POOL;

    use super::*;

    #[test]
    fn db() {
        let conn = POOL
            .connection()
            .expect("Failed to get database connection from the pool");
        let login = Login::from(conn);

        let idx = login
            .create("email", "password")
            .expect("Failed to create login");

        assert_eq!(idx, 1);
    }
}
