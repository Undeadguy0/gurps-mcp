//! Загрузка входа расчёта из БД: реализация в gurps-db (`calc_input`), здесь — ошибки MCP.
use crate::db::Db;
use crate::error::ToolResult;
pub use gurps_db::calc_input::{REFS, query};

/// Входные данные расчёта для персонажа `character:<id>`.
pub async fn load(db: &Db, id: &str) -> ToolResult<super::model::Input> {
    Ok(gurps_db::calc_input::load(db, id).await?)
}
