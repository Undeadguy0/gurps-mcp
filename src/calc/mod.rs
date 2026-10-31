//! Расчёт персонажа вынесен в крейт gurps-calc (crates/gurps-calc); здесь — реэкспорт
//! и загрузчик входных данных из SurrealDB (loader), которому нужен доступ к БД.
pub use gurps_calc::*;

pub mod loader;