//! Заклинания: `SpellDef` <-> `spell`.

use super::{skills, Ctx, Entity};
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::SpellDef;
use crate::sql::{Rec, Sv};
use serde_json::Value as J;

/// Заглушка обязательных полей, которых нет в листе.
pub const UNKNOWN: &str = "?";

pub fn college_rec(name: &str) -> Rec {
    Rec::code("ref_college", &ids::slug(name))
}

pub fn to_db(ctx: &mut Ctx, d: &SpellDef) -> anyhow::Result<Entity> {
    let rec = ctx.rec("spell", &d.id)?;
    let what = format!("spell {}", d.id);
    if d.attribute != "IQ" {
        ctx.gap("spell.attribute ≠ IQ: в схеме нет (заклинания всегда от IQ)", what.clone());
    }
    if d.college.is_empty() {
        anyhow::bail!("{what}: нет college, а схема требует хотя бы одну школу");
    }
    if d.prerequisites.as_deref().is_some_and(|p| !p.trim().is_empty()) {
        ctx.gap(
            "spell.prerequisites (текст): не переносится в prereqs (дерево), отброшено",
            what.clone(),
        );
    }
    ctx.gap(
        "spell.casting_time/duration/cost: в листе нет, записана заглушка raw = \"?\" (duration.kind = special)",
        what.clone(),
    );
    let raw = || Sv::obj().with("raw", Sv::str(UNKNOWN));
    let mut c = Sv::obj()
        .with("colleges", Sv::Arr(d.college.iter().map(|n| Sv::Rec(college_rec(n))).collect()))
        .with("classes", Sv::Arr(vec![Sv::Rec(Rec::code("ref_spell_class", &ctx.spell_class))]))
        .with("difficulty", skills::diff_rec(d.difficulty))
        .with("casting_time", raw())
        .with("duration", raw().with("kind", Sv::str("special")))
        .with("cost", raw());
    super::set_page(ctx, &mut c, d.page.as_deref(), &what);
    let mut e = Entity::new(rec, c);
    e.tr = ctx.texts("spell", &d.id, &d.name, d.description.as_ref());
    Ok(e)
}

/// `college_names`: код ref_college -> английское имя (из translation), иначе код.
pub fn from_db(j: &J, report: &mut Report, college_names: &std::collections::BTreeMap<String, String>) -> SpellDef {
    let id = super::kebab_sid(j);
    let what = format!("spell {id}");
    let mut lost = vec![];
    for k in ["casting_time", "duration", "cost"] {
        if j.get(k).and_then(|v| jsonx::s(v, "raw")).is_some_and(|r| r != UNKNOWN) {
            lost.push(k);
        }
    }
    for k in ["maintain", "resist", "prereqs"] {
        if j.get(k).is_some_and(|v| !v.is_null()) {
            lost.push(k);
        }
    }
    if jsonx::recs(j, "classes").iter().any(|c| c.code_str() != Some("regular")) {
        lost.push("classes");
    }
    for k in lost {
        report.gap(Dir::Export, format!("spell.{k}: в листе нет"), what.clone());
    }
    let college = jsonx::recs(j, "colleges")
        .iter()
        .filter_map(|r| r.code_str())
        .map(|c| {
            college_names.get(c).cloned().unwrap_or_else(|| {
                report.gap(Dir::Export, "spell.colleges: у ref_college нет имени en, выведен код", c);
                c.to_string()
            })
        })
        .collect();
    SpellDef {
        name: super::name_of(j, report, &what),
        attribute: "IQ".into(),
        difficulty: skills::diff_of(j, report, &what),
        college,
        prerequisites: None,
        page: super::page_of(j),
        description: super::desc_of(j),
        id,
    }
}
