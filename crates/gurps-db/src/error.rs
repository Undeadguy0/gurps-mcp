//! Ошибки слоя БД (без привязки к MCP; сервер отображает их в свои ошибки).

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("не найдено: {0}")]
    NotFound(String),
    #[error("неверный аргумент: {0}")]
    Invalid(String),
    #[error("подключение к {host}: {msg}")]
    Connect { host: String, msg: String },
    #[error("ошибка SurrealDB: {0}")]
    Query(String),
    #[error("разбор ответа БД: {0}")]
    Decode(String),
}

impl From<surrealdb::Error> for DbError {
    fn from(e: surrealdb::Error) -> Self {
        DbError::Query(e.to_string())
    }
}

pub type DbResult<T> = Result<T, DbError>;