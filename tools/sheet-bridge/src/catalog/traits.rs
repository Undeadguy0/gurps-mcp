//! Черты: `TraitDef` <-> `trait`.

use super::{features, Ctx, Entity};
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::{TraitCategory, TraitDef};
use crate::sql::Sv;
use serde_json::Value as J;

pub fn to_db(ctx: &mut Ctx, d: &TraitDef) -> anyhow::Result<Entity> {
    let rec = ctx.rec("trait", &d.id)?;
    let what = format!("trait {}", d.id);
    let kind = match d.category {
        TraitCategory::Advantage => "adv",
        TraitCategory::Disadvantage => "disadv",
        TraitCategory::Quirk => "quirk",
        TraitCategory::Perk => "perk",
    };
    let ty = match (d.cost, d.cost_per_level) {
        (_, 0) => "fixed",
        (0, _) => "per_level",
        _ => "base_plus_level",
    };
    let mut cost = Sv::obj().with("type", Sv::str(ty)).with("base", Sv::Int(d.cost as i64));
    if d.cost_per_level != 0 {
        cost.set("per_level", Sv::Int(d.cost_per_level as i64));
        if let Some(m) = d.max_levels {
            cost.set("max_level", Sv::Int(m as i64));
        }
    } else if d.max_levels.is_some() {
        ctx.gap("trait.max_levels без cost_per_level: отброшено", what.clone());
    }
    if d.min_levels.is_some() {
        ctx.gap("trait.min_levels: в схеме нет", what.clone());
    }
    let mut c = Sv::obj().with("kind", Sv::str(kind)).with("cost", cost);
    super::set_page(ctx, &mut c, d.page.as_deref(), &what);
    let fs = features::to_db(ctx, &d.features, &what);
    c.set("features", Sv::Arr(fs));
    let mut e = Entity::new(rec, c);
    e.tr = ctx.texts("trait", &d.id, &d.name, d.description.as_ref());
    Ok(e)
}

pub fn from_db(j: &J, report: &mut Report, colleges: &std::collections::BTreeMap<String, String>) -> TraitDef {
    let id = super::kebab_sid(j);
    let what = format!("trait {id}");
    let category = match jsonx::s(j, "kind") {
        Some("disadv") => TraitCategory::Disadvantage,
        Some("quirk") => TraitCategory::Quirk,
        Some("perk") => TraitCategory::Perk,
        _ => TraitCategory::Advantage,
    };
    let cost = j.get("cost").cloned().unwrap_or(J::Null);
    for (k, topic) in [
        ("self_control", "trait.self_control: в листе нет (зашито в id/имя)"),
        ("prereqs", "trait.prereqs: в листе нет"),
    ] {
        if j.get(k).is_some_and(|v| !v.is_null()) {
            report.gap(Dir::Export, topic, what.clone());
        }
    }
    for k in ["categories", "modifiers_allowed"] {
        if !jsonx::arr(j, k).is_empty() {
            report.gap(Dir::Export, format!("trait.{k}: в листе нет"), what.clone());
        }
    }
    TraitDef {
        name: super::name_of(j, report, &what),
        category,
        cost: jsonx::i(&cost, "base").unwrap_or(0) as i32,
        cost_per_level: jsonx::i(&cost, "per_level").unwrap_or(0) as i32,
        max_levels: jsonx::i(&cost, "max_level").map(|x| x as u32),
        min_levels: None,
        page: super::page_of(j),
        features: features::from_db(j, report, colleges, &what),
        description: super::desc_of(j),
        id,
    }
}
