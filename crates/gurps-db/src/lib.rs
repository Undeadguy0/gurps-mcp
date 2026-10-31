//! gurps-db: единый слой доступа к SurrealDB GURPS (схема Data/database — первична).
//!
//! - [`config`] / [`db`] — подключение (env SURREALDB_*) и запросы, возвращающие JSON;
//! - [`calc_input`] — вход расчёта персонажа для `gurps-calc` одним запросом;
//! - [`catalog`] — чтение каталога (только чтение): паки, записи `(pack, sid)`, переводы, ref_*;
//! - `testkit` (feature) — временная БД в памяти для тестов.
//!
//! Используется MCP-сервером (gurps-mcp) и редактором gurps-sheet (через path-зависимость).
pub mod calc_input;
pub mod catalog;
pub mod config;
pub mod db;
pub mod error;
#[cfg(feature = "testkit")]
pub mod testkit;

pub use config::DbConfig;
pub use db::{Db, Vars};
pub use error::{DbError, DbResult};
pub use gurps_calc;
