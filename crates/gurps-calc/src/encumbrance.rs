//! Нагрузка (таблица `ref_encumbrance`) и активные защиты (Dodge, лучшее Parry).
//!
//! - уровень: первая строка (по `level`), где перенос <= `max_load_bl` × Basic Lift;
//!   больше последней строки — перегруз (двигаться нельзя, Move 0);
//! - Move = floor(база × `move_mult`), не ниже `ref_rule:encumbrance_move_min`, если база не ниже его;
//!   база — атрибут `ref_rule:move_attribute`;
//! - Dodge = floor(атрибут `ref_rule:dodge_attribute`) + `ref_rule:dodge_base` + `dodge_mod`,
//!   при нагрузке не ниже `ref_rule:dodge_min`; бонусы защит (`defense_bonus`) — в [`Defenses`].
use super::Missing;
use super::attacks::Attack;
use super::attributes::Derived;
use super::features::Effects;
use super::model::{EncumbranceRow, Input, Refs};
use serde::Serialize;

const EPS: f64 = 1e-9;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Level {
    pub code: String,
    pub level: i64,
    pub max_weight: Option<f64>,
    pub move_value: Option<i64>,
    pub dodge: Option<i64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Encumbrance {
    pub carried_weight: f64,
    pub basic_lift: Option<f64>,
    /// Текущий уровень (`ref_encumbrance.level`) и его код.
    pub level: Option<i64>,
    pub code: Option<String>,
    /// Перенос больше последнего уровня таблицы.
    pub overloaded: bool,
    pub base_move: Option<f64>,
    /// Dodge без нагрузки и без бонусов защит.
    pub base_dodge: Option<i64>,
    pub move_value: Option<i64>,
    /// Dodge под текущей нагрузкой (без бонусов защит).
    pub dodge: Option<i64>,
    pub table: Vec<Level>,
}

/// Строки таблицы по возрастанию уровня.
pub fn rows(refs: &Refs) -> Vec<&EncumbranceRow> {
    let mut r: Vec<_> = refs.encumbrance.iter().collect();
    r.sort_by_key(|x| x.level);
    r
}

/// Строка уровня для веса; `true` — перегруз (возвращается последняя строка).
pub fn level_for(refs: &Refs, weight: f64, bl: f64) -> Option<(&EncumbranceRow, bool)> {
    let r = rows(refs);
    r.iter()
        .find(|x| weight <= bl * x.max_load_bl + EPS)
        .map(|x| (*x, false))
        .or_else(|| r.last().map(|x| (*x, true)))
}

pub fn move_for(refs: &Refs, base: f64, row: &EncumbranceRow, miss: &mut Missing) -> i64 {
    let m = (base * row.move_mult + EPS).floor() as i64;
    match refs.value("encumbrance_move_min") {
        Some(min) if base >= min => m.max(min as i64),
        Some(_) => m.max(0),
        None => {
            miss.add("ref_rule:encumbrance_move_min");
            m.max(0)
        }
    }
}

pub fn dodge_for(refs: &Refs, base: i64, row: &EncumbranceRow, miss: &mut Missing) -> i64 {
    let d = base + row.dodge_mod;
    if row.dodge_mod == 0 {
        return d;
    }
    match refs.value("dodge_min") {
        Some(min) => d.max(min as i64),
        None => {
            miss.add("ref_rule:dodge_min");
            d
        }
    }
}

/// floor(атрибут Dodge) + dodge_base.
pub fn base_dodge(refs: &Refs, attrs: &Derived, miss: &mut Missing) -> Option<i64> {
    let attr = refs.text("dodge_attribute");
    let base = refs.value("dodge_base");
    if attr.is_none() {
        miss.add("ref_rule:dodge_attribute");
    }
    if base.is_none() {
        miss.add("ref_rule:dodge_base");
    }
    let v = attrs.get(attr?);
    if v.is_none() {
        miss.add(format!("ref_attribute:{}", attr?));
    }
    Some((v? + EPS).floor() as i64 + base? as i64)
}

pub fn compute(inp: &Input, attrs: &Derived, miss: &mut Missing) -> Encumbrance {
    let refs = &inp.refs;
    if refs.encumbrance.is_empty() {
        miss.add("ref_encumbrance");
    }
    let base_move = match refs.text("move_attribute") {
        Some(a) => attrs.get(a),
        None => {
            miss.add("ref_rule:move_attribute");
            None
        }
    };
    let base_dodge = base_dodge(refs, attrs, miss);
    let w = inp.carried_weight.unwrap_or(0.0);
    let bl = attrs.basic_lift;
    let table = rows(refs)
        .into_iter()
        .map(|r| Level {
            code: r.code.clone(),
            level: r.level,
            max_weight: bl.map(|b| b * r.max_load_bl),
            move_value: base_move.map(|m| move_for(refs, m, r, miss)),
            dodge: base_dodge.map(|d| dodge_for(refs, d, r, miss)),
        })
        .collect();
    let cur = bl.and_then(|b| level_for(refs, w, b));
    let overloaded = cur.is_some_and(|c| c.1);
    Encumbrance {
        carried_weight: w,
        basic_lift: bl,
        level: cur.map(|c| c.0.level),
        code: cur.map(|c| c.0.code.clone()),
        overloaded,
        base_move,
        base_dodge,
        move_value: match (cur, base_move) {
            (Some(_), _) if overloaded => Some(0),
            (Some((r, _)), Some(m)) => Some(move_for(refs, m, r, miss)),
            _ => None,
        },
        dodge: cur
            .zip(base_dodge)
            .map(|((r, _), d)| dodge_for(refs, d, r, miss)),
        table,
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Defenses {
    /// Dodge под нагрузкой + бонус `defense_bonus(dodge)`.
    pub dodge: Option<i64>,
    /// Лучшее Parry: среди атак экипированных предметов, а если их нет — природных.
    pub parry: Option<i64>,
    /// sid атаки (и предмет), давшей лучшее Parry.
    pub parry_from: Option<String>,
    pub parry_item: Option<String>,
    /// Бонусы защит из эффектов: dodge/parry/block.
    pub dodge_bonus: i64,
    pub parry_bonus: i64,
    pub block_bonus: i64,
    // TODO(Block): правила Block не заведены в БД — ждём ответа Мастера; не считаем и не хардкодим.
}

pub fn defenses(enc: &Encumbrance, attacks: &[Attack], fx: &Effects) -> Defenses {
    let best = |armed: bool| {
        attacks
            .iter()
            .filter(|a| a.item.is_some() == armed)
            .filter_map(|a| a.parry.map(|p| (p, a)))
            .max_by_key(|x| x.0)
    };
    let p = best(true).or_else(|| best(false));
    let dodge_bonus = fx.defense("dodge");
    Defenses {
        dodge: enc.dodge.map(|d| d + dodge_bonus),
        parry: p.map(|x| x.0),
        parry_from: p.map(|x| x.1.sid.clone()),
        parry_item: p.and_then(|x| x.1.item.clone()),
        dodge_bonus,
        parry_bonus: fx.defense("parry"),
        block_bonus: fx.defense("block"),
    }
}
