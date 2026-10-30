//! Урон от ST: таблица `ref_st_damage` + правила `ref_rule` для значений между строками и выше таблицы.
//!
//! - точное совпадение `st` — строка таблицы;
//! - между строками — режим `ref_rule:st_damage_lookup` (`floor`: строка с наибольшим `st` <= ST);
//! - выше последней строки — последняя строка + `st_damage_extra_dice` кубиков к thr и sw
//!   за каждые полные `st_damage_extra_st` ST сверх неё.
use super::Missing;
use super::model::{Dice, Refs, StDamageRow};

pub const LOOKUP: &str = "st_damage_lookup";
pub const EXTRA_ST: &str = "st_damage_extra_st";
pub const EXTRA_DICE: &str = "st_damage_extra_dice";

/// (thr, sw) для ST; при нехватке данных — `None` и запись в `miss`.
pub fn lookup(refs: &Refs, st: i64, miss: &mut Missing) -> Option<(Dice, Dice)> {
    let rows = &refs.st_damage;
    if let Some(r) = rows.iter().find(|r| r.st == st) {
        return Some((r.thr, r.sw));
    }
    let below = rows.iter().filter(|r| r.st < st).max_by_key(|r| r.st);
    let Some(below) = below else {
        miss.add(format!("ref_st_damage для ST {st}"));
        return None;
    };
    if rows.iter().any(|r| r.st > st) {
        between(refs, below, miss)
    } else {
        above(refs, below, st, miss)
    }
}

fn between(refs: &Refs, below: &StDamageRow, miss: &mut Missing) -> Option<(Dice, Dice)> {
    match refs.text(LOOKUP) {
        Some("floor") => Some((below.thr, below.sw)),
        Some(other) => {
            miss.add(format!("ref_rule:{LOOKUP} (неизвестный режим {other:?})"));
            None
        }
        None => {
            miss.add(format!("ref_rule:{LOOKUP}"));
            None
        }
    }
}

fn above(refs: &Refs, top: &StDamageRow, st: i64, miss: &mut Missing) -> Option<(Dice, Dice)> {
    let step = refs.value(EXTRA_ST).filter(|v| *v > 0.0);
    let dice = refs.value(EXTRA_DICE);
    if step.is_none() {
        miss.add(format!("ref_rule:{EXTRA_ST}"));
    }
    if dice.is_none() {
        miss.add(format!("ref_rule:{EXTRA_DICE}"));
    }
    let (step, dice) = (step?, dice?);
    let extra = (((st - top.st) as f64 / step).floor() * dice) as i64;
    let add = |d: Dice| Dice {
        dice: d.dice + extra,
        add: d.add,
    };
    Some((add(top.thr), add(top.sw)))
}
