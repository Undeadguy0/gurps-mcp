//! Помощник тестов: временная БД из gurps-db (`surreal start memory` + surrealkit sync/seed).
#![allow(dead_code, unused_imports)]
pub use gurps_db::testkit::{Server, db_dir, start};