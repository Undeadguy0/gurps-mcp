//! Входные данные расчёта (форма, которую отдаёт `loader`). Ссылки — ключи (sid/code).
use serde::Deserialize;
use serde_json::Value as J;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AttrDef {
    pub code: String,
    pub kind: String,
    #[serde(default)]
    pub base_value: Option<f64>,
    #[serde(default)]
    pub base_from: Vec<String>,
    #[serde(default)]
    pub base_divisor: Option<f64>,
    #[serde(default)]
    pub cost_per_level: Option<i64>,
    #[serde(default = "one")]
    pub step: f64,
}

fn one() -> f64 {
    1.0
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DiffDef {
    pub code: String,
    #[serde(default)]
    pub base_mod: Option<i64>,
    #[serde(default)]
    pub technique_costs: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct SkillCostRow {
    pub points: i64,
    pub level_offset: i64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize, serde::Serialize)]
pub struct Dice {
    pub dice: i64,
    #[serde(rename = "mod")]
    pub add: i64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct StDamageRow {
    pub st: i64,
    pub thr: Dice,
    pub sw: Dice,
}

/// Уровень нагрузки (`ref_encumbrance`).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct EncumbranceRow {
    pub code: String,
    pub level: i64,
    pub max_load_bl: f64,
    pub move_mult: f64,
    pub dodge_mod: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RuleRow {
    pub code: String,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub text: Option<String>,
}

/// Справочники расчёта.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Refs {
    #[serde(default)]
    pub attributes: Vec<AttrDef>,
    #[serde(default)]
    pub difficulties: Vec<DiffDef>,
    #[serde(default)]
    pub skill_costs: Vec<SkillCostRow>,
    #[serde(default)]
    pub st_damage: Vec<StDamageRow>,
    #[serde(default)]
    pub rules: Vec<RuleRow>,
    #[serde(default)]
    pub encumbrance: Vec<EncumbranceRow>,
}

impl Refs {
    pub fn attr(&self, code: &str) -> Option<&AttrDef> {
        self.attributes.iter().find(|a| a.code == code)
    }
    pub fn diff(&self, code: &str) -> Option<&DiffDef> {
        self.difficulties.iter().find(|d| d.code == code)
    }
    pub fn value(&self, code: &str) -> Option<f64> {
        self.rules
            .iter()
            .find(|r| r.code == code)
            .and_then(|r| r.value)
    }
    pub fn text(&self, code: &str) -> Option<&str> {
        self.rules
            .iter()
            .find(|r| r.code == code)
            .and_then(|r| r.text.as_deref())
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AttrLevel {
    pub code: String,
    pub levels: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TraitCost {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub base: i64,
    #[serde(default)]
    pub per_level: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ModVal {
    /// enh | lim
    #[serde(rename = "type")]
    pub kind: String,
    /// % | pts | per_level
    pub value_kind: String,
    pub amount: f64,
    #[serde(default)]
    pub levels: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OwnedTrait {
    pub sid: String,
    #[serde(default)]
    pub level: Option<i64>,
    pub cost: TraitCost,
    #[serde(default)]
    pub modifiers: Vec<ModVal>,
    #[serde(default)]
    pub features: Vec<J>,
    #[serde(default)]
    pub prereqs: Option<J>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OwnedSkill {
    pub sid: String,
    pub attribute: String,
    pub difficulty: String,
    pub points: i64,
    #[serde(default)]
    pub spec: Option<String>,
    #[serde(default)]
    pub features: Vec<J>,
    #[serde(default)]
    pub prereqs: Option<J>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OwnedTechnique {
    pub sid: String,
    pub parent_skill: String,
    pub difficulty: String,
    pub default_mod: i64,
    #[serde(default)]
    pub max_mod: Option<i64>,
    pub points: i64,
    #[serde(default)]
    pub features: Vec<J>,
    #[serde(default)]
    pub prereqs: Option<J>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OwnedSpell {
    pub sid: String,
    #[serde(default)]
    pub colleges: Vec<String>,
    pub difficulty: String,
    pub points: i64,
    #[serde(default)]
    pub prereqs: Option<J>,
}

/// Умолчание навыка: от атрибута (`attribute`) или от навыка (`skill`).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DefaultRow {
    /// Навык, у которого есть умолчание.
    pub skill: String,
    #[serde(default)]
    pub attribute: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(rename = "mod")]
    pub modifier: i64,
}

/// Навык каталога (для умолчаний и атак по навыкам, которых у персонажа нет).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CatalogSkill {
    pub sid: String,
    pub attribute: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DamageDef {
    #[serde(default)]
    pub base: Option<String>,
    #[serde(default)]
    pub dice: Option<i64>,
    #[serde(default, rename = "mod")]
    pub add: i64,
    #[serde(default, rename = "type")]
    pub damage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AttackDefault {
    #[serde(default)]
    pub attribute: Option<String>,
    #[serde(default)]
    pub skill: Option<String>,
    #[serde(rename = "mod")]
    pub modifier: i64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct SkillDamageBonus {
    pub min_relative: i64,
    pub per_die: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AttackModeDef {
    /// sid шаблона (attack_mode) или `item_sid/имя` для режимов предметов.
    pub sid: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub natural: bool,
    #[serde(default)]
    pub skill: Option<String>,
    #[serde(default)]
    pub damage: DamageDef,
    #[serde(default)]
    pub reach: Vec<i64>,
    #[serde(default)]
    pub parry: Option<i64>,
    #[serde(default)]
    pub defaults: Vec<AttackDefault>,
    #[serde(default)]
    pub skill_modifier: Option<i64>,
    #[serde(default)]
    pub skill_damage_bonus: Option<SkillDamageBonus>,
    #[serde(default)]
    pub technique: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OwnedItem {
    pub sid: String,
    #[serde(default)]
    pub equipped: bool,
    #[serde(default)]
    pub features: Vec<J>,
    #[serde(default)]
    pub attack_modes: Vec<AttackModeDef>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LedgerRow {
    pub kind: String,
    pub delta: i64,
}

/// Всё, что нужно для расчёта одного персонажа.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Input {
    #[serde(default)]
    pub refs: Refs,
    #[serde(default)]
    pub attributes: Vec<AttrLevel>,
    #[serde(default)]
    pub current_hp: Option<i64>,
    #[serde(default)]
    pub current_fp: Option<i64>,
    /// Переносимый вес (для нагрузки), в единицах веса справочника.
    #[serde(default)]
    pub carried_weight: Option<f64>,
    #[serde(default)]
    pub traits: Vec<OwnedTrait>,
    #[serde(default)]
    pub skills: Vec<OwnedSkill>,
    #[serde(default)]
    pub techniques: Vec<OwnedTechnique>,
    #[serde(default)]
    pub spells: Vec<OwnedSpell>,
    #[serde(default)]
    pub items: Vec<OwnedItem>,
    #[serde(default)]
    pub defaults: Vec<DefaultRow>,
    #[serde(default)]
    pub catalog_skills: Vec<CatalogSkill>,
    #[serde(default)]
    pub natural_attacks: Vec<AttackModeDef>,
    #[serde(default)]
    pub ledger: Vec<LedgerRow>,
}
