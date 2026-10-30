//! Модификаторы черт: `ModifierDef` <-> `modifier`.

use super::{Ctx, Entity};
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::ModifierDef;
use crate::sql::Sv;
use serde_json::Value as J;

pub fn to_db(ctx: &mut Ctx, d: &ModifierDef) -> anyhow::Result<Entity> {
    let rec = ctx.rec("modifier", &d.id)?;
    let what = format!("modifier {}", d.id);
    let ty = if d.cost_percent > 0 { "enh" } else { "lim" };
    let mut c = Sv::obj().with("type", Sv::str(ty)).with(
        "value",
        Sv::obj().with("kind", Sv::str("%")).with("amount", Sv::Int(d.cost_percent as i64)),
    );
    super::set_page(ctx, &mut c, d.page.as_deref(), &what);
    let mut e = Entity::new(rec, c);
    e.tr = ctx.texts("modifier", &d.id, &d.name, d.description.as_ref());
    Ok(e)
}

pub fn from_db(j: &J, report: &mut Report) -> ModifierDef {
    let id = super::kebab_sid(j);
    let what = format!("modifier {id}");
    let v = j.get("value").cloned().unwrap_or(J::Null);
    if jsonx::s(&v, "kind") != Some("%") {
        report.gap(
            Dir::Export,
            "modifier.value.kind pts/per_level: в листе только проценты (взято amount как %)",
            what.clone(),
        );
    }
    if j.get("levels").is_some_and(|x| !x.is_null()) {
        report.gap(Dir::Export, "modifier.levels: в листе нет", what.clone());
    }
    if !jsonx::arr(j, "incompatible").is_empty() {
        report.gap(Dir::Export, "modifier.incompatible: в листе нет", what.clone());
    }
    let mut pct = jsonx::f(&v, "amount").unwrap_or(0.0).round() as i32;
    if jsonx::s(j, "type") == Some("lim") && pct > 0 {
        pct = -pct;
    }
    ModifierDef {
        name: super::name_of(j, report, &what),
        cost_percent: pct,
        page: super::page_of(j),
        description: super::desc_of(j),
        id,
    }
}
