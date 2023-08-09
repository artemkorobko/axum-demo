use std::ops::Deref;

use r2d2_sqlite::rusqlite;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl rusqlite::types::ToSql for Uuid {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let bytes = self.as_bytes().to_vec();
        let value = rusqlite::types::Value::Blob(bytes);
        Ok(rusqlite::types::ToSqlOutput::Owned(value))
    }
}

impl rusqlite::types::FromSql for Uuid {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let bytes = <[u8; 16]>::column_result(value)?;
        let value = u128::from_be_bytes(bytes);
        let id = uuid::Uuid::from_u128(value);
        Ok(Self::from(id))
    }
}
