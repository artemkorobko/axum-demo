use r2d2_sqlite::rusqlite;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DateTimeUtc(chrono::DateTime<chrono::Utc>);

impl DateTimeUtc {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl Deref for DateTimeUtc {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl rusqlite::types::ToSql for DateTimeUtc {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let millis = self.timestamp_millis();
        let value = rusqlite::types::Value::Integer(millis);
        Ok(rusqlite::types::ToSqlOutput::Owned(value))
    }
}

impl rusqlite::types::FromSql for DateTimeUtc {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let millis = <i64>::column_result(value)?;
        let date_time = chrono::NaiveDateTime::from_timestamp_millis(millis)
            .map(|naive| chrono::DateTime::from_utc(naive, chrono::Utc))
            .ok_or(rusqlite::types::FromSqlError::InvalidType)?;
        Ok(Self(date_time))
    }
}
