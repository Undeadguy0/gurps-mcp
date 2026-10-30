//! Атрибуты и производные: итоговые значения, Basic Lift, урон от ST, бонусы защит.
use super::Missing;
use super::features::Effects;
use super::model::{Dice, Input, Refs};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Derived {
    /// code -> итоговое значение (база + купленные шаги + эффекты).
    pub values: BTreeMap<String, f64>,
    pub basic_lift: Option<f64>,
    pub lifting_st: Option<f64>,
    pub striking_st: Option<f64>,
    pub thrust: Option<Dice>,
    pub swing: Option<Dice>,
    pub current_hp: Option<i64>,
    pub current_fp: Option<i64>,
    /// Бонусы к защитам из эффектов: dodge/parry/block.
    pub defense_bonus: BTreeMap<String, i64>,
}

impl Derived {
    pub fn get(&self, code: &str) -> Option<f64> {
        self.values.get(code).copied()
    }
}

/// Значения атрибутов в порядке зависимостей (`base_from`).
pub fn values(
    refs: &Refs,
    levels: &BTreeMap<String, i64>,
    fx: &Effects,
    miss: &mut Missing,
) -> BTreeMap<String, f64> {
    let mut out: BTreeMap<String, f64> = BTreeMap::new();
    let mut pending: Vec<_> = refs.attributes.iter().collect();
    while !pending.is_empty() {
        let before = pending.len();
        pending.retain(|a| {
            if !a.base_from.iter().all(|b| out.contains_key(b)) {
                return true;
            }
            let base = if a.base_from.is_empty() {
                match a.base_value {
                    Some(v) => v,
                    None => {
                        miss.add(format!("ref_attribute:{}.base_value", a.code));
                        return false;
                    }
                }
            } else {
                let sum: f64 = a.base_from.iter().map(|b| out[b]).sum();
                let v = sum / a.base_divisor.unwrap_or(1.0);
                // у атрибутов с целым шагом дробная часть базы отбрасывается
                if a.step.fract() == 0.0 { v.floor() } else { v }
            };
            let (add, set) = fx.attribute(&a.code);
            let lv = levels.get(&a.code).copied().unwrap_or(0) as f64;
            out.insert(a.code.clone(), set.unwrap_or(base + lv * a.step + add));
            false
        });
        if pending.len() == before {
            for a in pending {
                miss.add(format!(
                    "ref_attribute:{}.base_from (цикл или нет атрибута)",
                    a.code
                ));
            }
            break;
        }
    }
    out
}

pub fn st_damage(refs: &Refs, st: i64, miss: &mut Missing) -> Option<(Dice, Dice)> {
    match refs.st_damage.iter().find(|r| r.st == st) {
        Some(r) => Some((r.thr, r.sw)),
        None => {
            miss.add(format!("ref_st_damage для ST {st}"));
            None
        }
    }
}

pub fn basic_lift(refs: &Refs, st: f64, miss: &mut Missing) -> Option<f64> {
    let Some(div) = refs.value("basic_lift_divisor") else {
        miss.add("ref_rule:basic_lift_divisor");
        return None;
    };
    let bl = st * st / div;
    match refs.value("basic_lift_round_min") {
        Some(m) if bl >= m => Some(bl.round()),
        Some(_) => Some(bl),
        None => {
            miss.add("ref_rule:basic_lift_round_min");
            Some(bl)
        }
    }
}

pub fn compute(inp: &Input, fx: &Effects, miss: &mut Missing) -> Derived {
    let levels: BTreeMap<String, i64> = inp
        .attributes
        .iter()
        .map(|a| (a.code.clone(), a.levels))
        .collect();
    let values = values(&inp.refs, &levels, fx, miss);
    let st = values.get("ST").copied();
    let lifting_st = st.map(|s| s + fx.lifting_st());
    let striking_st = st.map(|s| s + fx.striking_st());
    let basic_lift = lifting_st.and_then(|s| basic_lift(&inp.refs, s, miss));
    let dmg = striking_st.and_then(|s| st_damage(&inp.refs, s.floor() as i64, miss));
    let defense_bonus = ["dodge", "parry", "block"]
        .iter()
        .map(|d| (d.to_string(), fx.defense(d)))
        .collect();
    Derived {
        values,
        basic_lift,
        lifting_st,
        striking_st,
        thrust: dmg.map(|d| d.0),
        swing: dmg.map(|d| d.1),
        current_hp: inp.current_hp,
        current_fp: inp.current_fp,
        defense_bonus,
    }
}
