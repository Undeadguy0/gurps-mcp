//! Персонаж (`*.gurps.json`): `{ format: "gurps-sheet", version: 1, character }`.

use super::defs::*;
use serde::{Deserialize, Serialize};

pub const FORMAT: &str = "gurps-sheet";
pub const VERSION: u32 = 1;

fn is_blank(s: &Option<String>) -> bool {
    s.as_deref().is_none_or(|t| t.trim().is_empty())
}
fn one() -> u32 {
    1
}
fn yes() -> bool {
    true
}
fn is_true(b: &bool) -> bool {
    *b
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Attributes {
    pub st: i32,
    pub dx: i32,
    pub iq: i32,
    pub ht: i32,
    pub hp: i32,
    pub will: i32,
    pub per: i32,
    pub fp: i32,
    pub basic_speed_quarters: i32,
    pub basic_move: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitModifier {
    #[serde(flatten)]
    pub def: ModifierDef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trait {
    #[serde(flatten)]
    pub def: TraitDef,
    #[serde(default)]
    pub levels: u32,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<TraitModifier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    #[serde(flatten)]
    pub def: SkillDef,
    #[serde(default)]
    pub points: i32,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub specialization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Technique {
    #[serde(flatten)]
    pub def: TechniqueDef,
    #[serde(default)]
    pub points: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell {
    #[serde(flatten)]
    pub def: SpellDef,
    #[serde(default)]
    pub points: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    #[serde(flatten)]
    pub def: EquipmentDef,
    #[serde(default = "one")]
    pub quantity: u32,
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    pub carried: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<Equipment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    #[serde(flatten)]
    pub def: WeaponDef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "is_blank")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LedgerKind {
    Starting,
    SessionAward,
    GmBonus,
    Training,
    NewDisadvantage,
    Correction,
    Influence,
    ImpulseBuy,
    BuyOff,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: u32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub date: String,
    pub amount: i32,
    pub kind: LedgerKind,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegativeRounding {
    #[default]
    TowardPositive,
    ByMagnitude,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RulesOptions {
    pub negative_rounding: NegativeRounding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disadvantage_limit: Option<i32>,
}

impl RulesOptions {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp: Option<i32>,
}

impl Session {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomContent {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<TraitDef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<SkillDef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spells: Vec<SpellDef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub equipment: Vec<EquipmentDef>,
}

impl CustomContent {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Character {
    pub name: String,
    pub player: String,
    pub points_budget: i32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ledger: Vec<LedgerEntry>,
    #[serde(skip_serializing_if = "CustomContent::is_empty")]
    pub custom: CustomContent,
    pub attributes: Attributes,
    pub traits: Vec<Trait>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<Skill>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub techniques: Vec<Technique>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spells: Vec<Spell>,
    pub equipment: Vec<Equipment>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weapons: Vec<Weapon>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub notes: String,
    #[serde(skip_serializing_if = "RulesOptions::is_default")]
    pub rules: RulesOptions,
    #[serde(skip_serializing_if = "Session::is_default")]
    pub session: Session,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "New character".into(),
            player: String::new(),
            points_budget: 150,
            ledger: Vec::new(),
            custom: CustomContent::default(),
            attributes: Attributes::default(),
            traits: Vec::new(),
            skills: Vec::new(),
            techniques: Vec::new(),
            spells: Vec::new(),
            equipment: Vec::new(),
            weapons: Vec::new(),
            notes: String::new(),
            rules: RulesOptions::default(),
            session: Session::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterFile {
    pub format: String,
    pub version: u32,
    pub character: Character,
}

pub fn load_character(path: &std::path::Path) -> anyhow::Result<Character> {
    let s = std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;
    let f: CharacterFile = serde_json::from_str(&s).map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;
    if f.format != FORMAT {
        anyhow::bail!("{}: не файл gurps-sheet (format = {:?})", path.display(), f.format);
    }
    if f.version == 0 || f.version > VERSION {
        anyhow::bail!("{}: неподдерживаемая версия {}", path.display(), f.version);
    }
    Ok(f.character)
}

pub fn character_to_string(ch: &Character) -> anyhow::Result<String> {
    let f = CharacterFile {
        format: FORMAT.into(),
        version: VERSION,
        character: ch.clone(),
    };
    let mut s = serde_json::to_string_pretty(&f)?;
    s.push('\n');
    Ok(s)
}
