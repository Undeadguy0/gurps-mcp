//! Подключение к SurrealDB: общий слой gurps-db (ошибки -> ToolError через `From`).
pub use gurps_db::db::{Db, Vars};