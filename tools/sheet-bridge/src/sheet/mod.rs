//! Зеркало форматов gurps-sheet @ a5fbb42 (крейты gurps-core / gurps-plugins / gurps-io).
//! Типы повторяют serde-атрибуты оригинала, чтобы файлы читались и писались байт-в-байт
//! совместимо; расчётной логики здесь нет.

pub mod character;
pub mod defs;
pub mod ftl;
pub mod plugin;

pub use character::*;
pub use defs::*;
