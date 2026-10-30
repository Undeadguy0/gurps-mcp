//! Персонажи: файл gurps-sheet <-> `character` + рёбра + пак `custom_<id>`.

pub mod export;
pub mod import;
pub mod matcher;

/// Атрибуты листа <-> коды ref_attribute (Speed — в четвертях, шаг 0.25).
pub const ATTRS: [&str; 10] = ["ST", "DX", "IQ", "HT", "HP", "Will", "Per", "FP", "Speed", "Move"];

pub fn custom_pack(id: &str) -> String {
    format!("custom_{id}")
}

/// Таблицы рёбер персонажа.
pub const EDGES: [&str; 5] = ["has_trait", "has_skill", "has_technique", "has_spell", "owns_item"];
