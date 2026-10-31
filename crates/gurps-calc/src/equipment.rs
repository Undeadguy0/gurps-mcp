//! Снаряжение: вес и стоимость (всё / при себе), DR надетой брони.
use super::model::{ArmorLoc, Input};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ArmorRow {
    pub item: String,
    pub locations: Vec<ArmorLoc>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Equipment {
    pub total_weight: f64,
    pub total_cost: f64,
    pub carried_weight: f64,
    pub carried_cost: f64,
    /// Надетая (при себе) броня.
    pub armor: Vec<ArmorRow>,
}

pub fn compute(inp: &Input) -> Equipment {
    let mut e = Equipment::default();
    for it in &inp.items {
        let q = it.qty.max(0) as f64;
        let w = q * it.weight.unwrap_or(0.0);
        let c = q * it.cost.unwrap_or(0.0);
        e.total_weight += w;
        e.total_cost += c;
        if it.equipped {
            e.carried_weight += w;
            e.carried_cost += c;
            if !it.armor.is_empty() {
                e.armor.push(ArmorRow {
                    item: it.sid.clone(),
                    locations: it.armor.clone(),
                });
            }
        }
    }
    e
}
