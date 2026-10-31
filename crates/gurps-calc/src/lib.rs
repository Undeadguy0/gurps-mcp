//! Расчёт персонажа: чистые функции над данными из БД (константы — только из справочников).
//!
//! Загрузчик (`gurps_mcp::calc::loader`, в сервере) собирает [`model::Input`] одним запросом; остальные модули — чистый Rust.
pub mod attacks;
pub mod attributes;
pub mod encumbrance;
pub mod features;
pub mod model;
pub mod points;
pub mod prereq;
pub mod skills;
pub mod st_damage;

use model::Input;
use serde::Serialize;
use serde_json::Value as J;
use std::collections::BTreeSet;

/// Ключ записи из JSON-строки ссылки: `skill:['basic', 'x']` -> `x`, `ref_attribute:DX` -> `DX`.
pub fn key(v: &J) -> Option<String> {
    let s = v.as_str()?;
    let (_, rest) = s.split_once(':')?;
    let clean = |x: &str| {
        x.trim()
            .trim_matches(|c| matches!(c, '\'' | '"' | '`' | '⟨' | '⟩'))
            .to_string()
    };
    if let Some(inner) = rest.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
        inner.rsplit(',').next().map(clean)
    } else {
        Some(clean(rest))
    }
}

/// Сборщик недостающих констант справочников (для Мастера).
#[derive(Debug, Default, Clone, Serialize)]
pub struct Missing(pub BTreeSet<String>);

impl Missing {
    pub fn add(&mut self, what: impl Into<String>) {
        self.0.insert(what.into());
    }
}

/// Полный расчёт.
#[derive(Debug, Serialize)]
pub struct Computed {
    pub attributes: attributes::Derived,
    pub skills: Vec<skills::SkillLevel>,
    pub techniques: Vec<skills::SkillLevel>,
    pub spells: Vec<skills::SkillLevel>,
    pub attacks: Vec<attacks::Attack>,
    pub points: points::Points,
    pub prereq_violations: Vec<prereq::Violation>,
    pub encumbrance: encumbrance::Encumbrance,
    pub defenses: encumbrance::Defenses,
    /// Константы, которых нет в справочниках: расчёт по ним пропущен.
    pub missing_constants: Missing,
}

pub fn compute(inp: &Input) -> Computed {
    if !inp.rule_overrides.is_empty() {
        let mut i = inp.clone();
        i.refs = inp.refs.with_overrides(&inp.rule_overrides);
        i.rule_overrides.clear();
        return compute(&i);
    }
    let mut miss = Missing::default();
    let fx = features::Effects::collect(inp);
    let attrs = attributes::compute(inp, &fx, &mut miss);
    let sk = skills::Skills::compute(inp, &attrs, &fx, &mut miss);
    let attacks = attacks::compute(inp, &attrs, &sk, &fx, &mut miss);
    let enc = encumbrance::compute(inp, &attrs, &mut miss);
    let defenses = encumbrance::defenses(&enc, &attacks, &fx);
    let points = points::compute(inp, &mut miss);
    let ctx = prereq::Ctx::new(inp, &attrs, &sk, &mut miss);
    let prereq_violations = prereq::check_all(inp, &ctx);
    Computed {
        attributes: attrs,
        skills: sk.skills.clone(),
        techniques: sk.techniques.clone(),
        spells: sk.spells.clone(),
        attacks,
        points,
        prereq_violations,
        encumbrance: enc,
        defenses,
        missing_constants: miss,
    }
}
