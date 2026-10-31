//! Ошибки инструментов и их отображение в ошибки MCP.
use rmcp::ErrorData;

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("не найдено: {0}")]
    NotFound(String),
    #[error("неверные параметры: {0}")]
    InvalidParams(String),
    #[error("ошибка SurrealDB: {0}")]
    Db(String),
}

impl From<surrealdb::Error> for ToolError {
    fn from(e: surrealdb::Error) -> Self {
        ToolError::Db(e.to_string())
    }
}

impl From<ToolError> for ErrorData {
    fn from(e: ToolError) -> Self {
        match e {
            ToolError::NotFound(m) => ErrorData::resource_not_found(m, None),
            ToolError::InvalidParams(m) => ErrorData::invalid_params(m, None),
            ToolError::Db(m) => ErrorData::internal_error(m, None),
        }
    }
}

pub type ToolResult<T> = Result<T, ToolError>;
