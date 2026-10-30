//! Определения каталога (TOML `plugins/<id>/data/*.toml`).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Markdown-текст: строка или таблица по локалям.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalizedText {
    Plain(String),
    Localized(BTreeMap<String, String>),
}

impl LocalizedText {
    /// Пары (lang, text); `Plain` считается английским.
    pub fn by_lang(&self) -> Vec<(String, String)> {
        match self {
            LocalizedText::Plain(s) => vec![("en".into(), s.clone())],
            LocalizedText::Localized(m) => m.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TraitCategory {
    #[default]
    Advantage,
    Perk,
    Disadvantage,
    Quirk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Feature {
    AttributeBonus {
        target: String,
        amount: f64,
        #[serde(default)]
        per_level: bool,
    },
    LiftingStBonus {
        amount: f64,
        #[serde(default)]
        per_level: bool,
    },
    StrikingStBonus {
        amount: f64,
        #[serde(default)]
        per_level: bool,
    },
    DefenseBonus {
        amount: f64,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        defenses: Vec<String>,
        #[serde(default)]
        per_level: bool,
    },
    SpellBonus {
        amount: f64,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        colleges: Vec<String>,
        #[serde(default)]
        per_level: bool,
    },
    SkillBonus {
        skills: Vec<String>,
        amount: f64,
        #[serde(default)]
        per_level: bool,
    },
}

impl Feature {
    pub fn type_name(&self) -> &'static str {
        match self {
            Feature::AttributeBonus { .. } => "attribute_bonus",
            Feature::LiftingStBonus { .. } => "lifting_st_bonus",
            Feature::StrikingStBonus { .. } => "striking_st_bonus",
            Feature::DefenseBonus { .. } => "defense_bonus",
            Feature::SpellBonus { .. } => "spell_bonus",
            Feature::SkillBonus { .. } => "skill_bonus",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub category: TraitCategory,
    #[serde(default)]
    pub cost: i32,
    #[serde(default)]
    pub cost_per_level: i32,
    #[serde(default)]
    pub max_levels: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_levels: Option<u32>,
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub features: Vec<Feature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Difficulty {
    #[serde(rename = "E")]
    Easy,
    #[default]
    #[serde(rename = "A")]
    Average,
    #[serde(rename = "H")]
    Hard,
    #[serde(rename = "VH")]
    VeryHard,
}

impl Difficulty {
    pub fn key(self) -> &'static str {
        match self {
            Difficulty::Easy => "E",
            Difficulty::Average => "A",
            Difficulty::Hard => "H",
            Difficulty::VeryHard => "VH",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "E" => Difficulty::Easy,
            "A" => Difficulty::Average,
            "H" => Difficulty::Hard,
            "VH" => Difficulty::VeryHard,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkillDefault {
    Attribute { attribute: String, modifier: i32 },
    Skill { skill: String, modifier: i32 },
}

fn dx() -> String {
    "DX".into()
}
fn iq() -> String {
    "IQ".into()
}
fn hard() -> Difficulty {
    Difficulty::Hard
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillDef {
    pub id: String,
    pub name: String,
    #[serde(default = "dx")]
    pub attribute: String,
    #[serde(default)]
    pub difficulty: Difficulty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub defaults: Vec<SkillDefault>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub specialization_required: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specializations: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechniqueDef {
    pub id: String,
    pub name: String,
    pub skill: String,
    #[serde(default)]
    pub difficulty: Difficulty,
    pub modifier: i32,
    #[serde(default)]
    pub max: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpellDef {
    pub id: String,
    pub name: String,
    #[serde(default = "iq")]
    pub attribute: String,
    #[serde(default = "hard")]
    pub difficulty: Difficulty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub college: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EquipmentDef {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub weight: f64,
    #[serde(default)]
    pub cost: f64,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub container: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillDamageBonus {
    pub min_relative: i32,
    pub per_die: i32,
}

fn is_zero(v: &i32) -> bool {
    *v == 0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackDef {
    pub mode: String,
    pub damage: String,
    pub damage_type: String,
    #[serde(default)]
    pub reach: String,
    #[serde(default)]
    pub parry: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub skill_modifier: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technique: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub defaults: Vec<SkillDefault>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill_damage_bonus: Option<SkillDamageBonus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponDef {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equipment: Option<String>,
    pub skill: String,
    pub attacks: Vec<AttackDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifierDef {
    pub id: String,
    pub name: String,
    pub cost_percent: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedText>,
}

/// Содержимое одного или всех `data/*.toml` плагина.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DataFile {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<TraitDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<SkillDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub techniques: Vec<TechniqueDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spells: Vec<SpellDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub equipment: Vec<EquipmentDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weapons: Vec<WeaponDef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<ModifierDef>,
}

impl DataFile {
    pub fn extend(&mut self, o: DataFile) {
        self.traits.extend(o.traits);
        self.skills.extend(o.skills);
        self.techniques.extend(o.techniques);
        self.spells.extend(o.spells);
        self.equipment.extend(o.equipment);
        self.weapons.extend(o.weapons);
        self.modifiers.extend(o.modifiers);
    }

    pub fn is_empty(&self) -> bool {
        self == &DataFile::default()
    }
}
