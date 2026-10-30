//! Инструменты по темам. Каждая функция принимает &Db и параметры, возвращает JSON.
pub mod character;
pub mod entry;
pub mod packs;
pub mod search;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Каталожные таблицы, доступные инструментам.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CatalogTable {
    Skill,
    Trait,
    Item,
    Spell,
    Technique,
}

impl CatalogTable {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Skill => "skill",
            Self::Trait => "trait",
            Self::Item => "item",
            Self::Spell => "spell",
            Self::Technique => "technique",
        }
    }
}

/// Язык по умолчанию.
pub fn default_lang() -> String {
    "ru".into()
}

pub(crate) fn check_lang(lang: &str) -> crate::error::ToolResult<()> {
    if lang.len() == 2 && lang.bytes().all(|b| b.is_ascii_lowercase()) {
        Ok(())
    } else {
        Err(crate::error::ToolError::InvalidParams(format!(
            "lang: ожидается код из 2 букв, получено {lang:?}"
        )))
    }
}

pub(crate) fn check_key(name: &str, v: &str) -> crate::error::ToolResult<()> {
    if !v.is_empty()
        && v.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    {
        Ok(())
    } else {
        Err(crate::error::ToolError::InvalidParams(format!(
            "{name}: ожидается [a-z0-9_]+, получено {v:?}"
        )))
    }
}
