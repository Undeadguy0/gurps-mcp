//! Атаки: безоружные шаблоны (`attack_mode.natural`) и режимы надетых предметов + эффекты.
//!
//! Порядок: база урона (thr/sw от ударной ST или кубы) -> плоские бонусы -> `op = set`
//! (тип/досягаемость, напр. когти) -> бонусы «за кубик» последними.
use super::Missing;
use super::attributes::Derived;
use super::features::Effects;
use super::model::{AttackModeDef, Dice, Input};
use super::skills::Skills;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Attack {
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub skill: Option<String>,
    pub level: Option<i64>,
    pub damage: Option<Dice>,
    /// Текстом: `1d+2`.
    pub damage_text: Option<String>,
    pub damage_type: Option<String>,
    pub reach: Vec<i64>,
    pub parry: Option<i64>,
}

pub fn dice_text(d: Dice) -> String {
    match d.add {
        0 => format!("{}d", d.dice),
        a if a > 0 => format!("{}d+{a}", d.dice),
        a => format!("{}d{a}", d.dice),
    }
}

/// Уровень атаки: лучший из навыка, умолчаний атаки и техники; + skill_modifier.
pub fn level(m: &AttackModeDef, attrs: &Derived, sk: &Skills) -> Option<i64> {
    let mut cands: Vec<i64> = Vec::new();
    if let Some(s) = &m.skill {
        cands.extend(sk.level(s));
    }
    for d in &m.defaults {
        let base = match (&d.attribute, &d.skill) {
            (Some(a), _) => attrs.get(a).map(|v| v.floor() as i64),
            (_, Some(s)) => sk.level(s),
            _ => None,
        };
        cands.extend(base.map(|b| b + d.modifier));
    }
    if let Some(t) = &m.technique {
        cands.extend(sk.technique(t));
    }
    cands
        .into_iter()
        .max()
        .map(|l| l + m.skill_modifier.unwrap_or(0))
}

pub fn one(
    m: &AttackModeDef,
    item: Option<&str>,
    inp: &Input,
    attrs: &Derived,
    sk: &Skills,
    fx: &Effects,
    miss: &mut Missing,
) -> Attack {
    let lvl = level(m, attrs, sk);
    // база урона
    let base = match m.damage.base.as_deref() {
        Some("thr") => attrs.thrust,
        Some("sw") => attrs.swing,
        _ => m.damage.dice.map(|d| Dice { dice: d, add: 0 }),
    };
    let mut dmg = base.map(|b| Dice {
        dice: b.dice,
        add: b.add + m.damage.add,
    });
    let mut damage_type = m.damage.damage_type.clone();
    let mut reach = m.reach.clone();
    let mut per_die = 0i64;
    for e in fx.damage(&m.sid, m.skill.as_deref()) {
        if e.set {
            if let Some(t) = &e.damage_type {
                damage_type = Some(t.clone());
            }
            if let Some(r) = &e.reach {
                reach = r.clone();
            }
        } else if e.per_die {
            per_die += e.amount as i64;
        } else if let Some(d) = dmg.as_mut() {
            d.add += e.amount as i64;
        }
    }
    // бонус «за кубик» от уровня навыка относительно атрибута
    if let (Some(b), Some(s)) = (m.skill_damage_bonus, &m.skill) {
        let rel = sk
            .skills
            .iter()
            .chain(&sk.defaulted)
            .filter(|x| &x.sid == s)
            .filter_map(|x| x.relative)
            .max();
        if rel.is_some_and(|r| r >= b.min_relative) {
            per_die += b.per_die;
        }
    }
    if let Some(d) = dmg.as_mut() {
        d.add += per_die * d.dice;
    }
    // парирование: parry_base + уровень / parry_skill_divisor + parry атаки + defense_bonus(parry)
    let parry = match (m.parry, lvl) {
        (Some(p), Some(l)) => {
            let (b, div) = (
                inp.refs.value("parry_base"),
                inp.refs.value("parry_skill_divisor"),
            );
            if b.is_none() {
                miss.add("ref_rule:parry_base");
            }
            if div.is_none() {
                miss.add("ref_rule:parry_skill_divisor");
            }
            b.zip(div.filter(|d| *d > 0.0))
                .map(|(b, d)| b as i64 + (l as f64 / d).floor() as i64 + p + fx.defense("parry"))
        }
        _ => None,
    };
    Attack {
        sid: m.sid.clone(),
        item: item.map(str::to_string),
        name: m.name.clone(),
        skill: m.skill.clone(),
        level: lvl,
        damage_text: dmg.map(dice_text),
        damage: dmg,
        damage_type,
        reach,
        parry,
    }
}

pub fn compute(
    inp: &Input,
    attrs: &Derived,
    sk: &Skills,
    fx: &Effects,
    miss: &mut Missing,
) -> Vec<Attack> {
    let mut out: Vec<Attack> = inp
        .natural_attacks
        .iter()
        .map(|m| one(m, None, inp, attrs, sk, fx, miss))
        .collect();
    for i in inp.items.iter().filter(|i| i.equipped) {
        out.extend(
            i.attack_modes
                .iter()
                .map(|m| one(m, Some(&i.sid), inp, attrs, sk, fx, miss)),
        );
    }
    out
}
