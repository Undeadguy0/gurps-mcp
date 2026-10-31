//! Сумма потраченных очков (атрибуты, черты с модификаторами, навыки, техники, заклинания) против журнала.
use super::model::{Input, OwnedTrait, Refs};
use super::Missing;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Points {
    pub attributes: i64,
    /// Часть `attributes` за базовые (ref_attribute.kind = basic).
    pub basic_attributes: i64,
    /// Часть `attributes` за вторичные (ref_attribute.kind = secondary).
    pub secondary_attributes: i64,
    pub advantages: i64,
    pub disadvantages: i64,
    pub skills: i64,
    pub techniques: i64,
    pub spells: i64,
    pub total: i64,
    /// Получено по журналу (все записи, кроме spend).
    pub ledger_budget: i64,
    /// Потрачено по журналу (−сумма spend).
    pub ledger_spent: i64,
    /// budget − total.
    pub unspent: i64,
    /// total совпадает с ledger_spent.
    pub matches_ledger: bool,
}

/// Режим округления отрицательной дробной стоимости черты: `toward_positive` (-7.5 -> -7) | `by_magnitude` (-7.5 -> -8).
pub const NEGATIVE_ROUNDING: &str = "trait_cost_negative_rounding";

/// Стоимость черты: база/уровни, затем модификаторы (% суммой с нижней границей, pts).
pub fn trait_cost(refs: &Refs, t: &OwnedTrait, miss: &mut Missing) -> i64 {
    let lvl = t.level.unwrap_or(0);
    let per = t.cost.per_level.unwrap_or(0);
    let base = match t.cost.kind.as_str() {
        "per_level" => per * lvl,
        "base_plus_level" => t.cost.base + per * lvl,
        _ => t.cost.base,
    };
    let mut pct = 0.0;
    let mut flat = 0.0;
    for m in &t.modifiers {
        let sign = if m.kind == "lim" && m.amount > 0.0 {
            -1.0
        } else {
            1.0
        };
        match m.value_kind.as_str() {
            "%" => pct += sign * m.amount,
            "per_level" => flat += sign * m.amount * m.levels.unwrap_or(1) as f64,
            _ => flat += sign * m.amount,
        }
    }
    if pct != 0.0 {
        match refs.value("limitation_floor_percent") {
            Some(f) => pct = pct.max(f),
            None => miss.add("ref_rule:limitation_floor_percent"),
        }
    }
    // дроби: положительные — вверх; отрицательные — по ref_rule:trait_cost_negative_rounding
    let v = (base as f64 + flat) * (1.0 + pct / 100.0);
    if v >= 0.0 || v.fract() == 0.0 {
        return v.ceil() as i64;
    }
    match refs.text(NEGATIVE_ROUNDING) {
        Some("toward_positive") => v.ceil() as i64,
        Some("by_magnitude") => v.floor() as i64,
        other => {
            miss.add(match other {
                Some(o) => format!("ref_rule:{NEGATIVE_ROUNDING} (неизвестный режим {o:?})"),
                None => format!("ref_rule:{NEGATIVE_ROUNDING}"),
            });
            v.ceil() as i64
        }
    }
}

pub fn compute(inp: &Input, miss: &mut Missing) -> Points {
    let mut p = Points::default();
    for a in &inp.attributes {
        let def = inp.refs.attr(&a.code);
        match def.and_then(|d| d.cost_per_level) {
            Some(c) => {
                p.attributes += a.levels * c;
                if def.is_some_and(|d| d.kind == "basic") {
                    p.basic_attributes += a.levels * c;
                } else {
                    p.secondary_attributes += a.levels * c;
                }
            }
            None => miss.add(format!("ref_attribute:{}.cost_per_level", a.code)),
        }
    }
    for t in &inp.traits {
        let c = trait_cost(&inp.refs, t, miss);
        if c >= 0 {
            p.advantages += c;
        } else {
            p.disadvantages += c;
        }
    }
    p.skills = inp.skills.iter().map(|s| s.points).sum();
    p.techniques = inp.techniques.iter().map(|s| s.points).sum();
    p.spells = inp.spells.iter().map(|s| s.points).sum();
    p.total = p.attributes + p.advantages + p.disadvantages + p.skills + p.techniques + p.spells;
    p.ledger_budget = inp
        .ledger
        .iter()
        .filter(|l| l.kind != "spend")
        .map(|l| l.delta)
        .sum();
    p.ledger_spent = -inp
        .ledger
        .iter()
        .filter(|l| l.kind == "spend")
        .map(|l| l.delta)
        .sum::<i64>();
    p.unspent = p.ledger_budget - p.total;
    p.matches_ledger = p.total == p.ledger_spent;
    p
}
