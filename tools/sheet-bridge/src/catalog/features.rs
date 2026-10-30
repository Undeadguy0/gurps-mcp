//! Особенности (features): лист `Feature` <-> объекты `features[]` схемы.

use super::Ctx;
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::Feature;
use crate::sql::{Rec, Sv};
use serde_json::Value as J;

fn base(kind: &str, amount: f64, per_level: bool) -> Sv {
    let mut o = Sv::obj().with("kind", Sv::str(kind)).with("amount", super::num(amount));
    if per_level {
        o.set("per_level", Sv::Bool(true));
    }
    o
}

pub fn to_db(ctx: &mut Ctx, fs: &[Feature], what: &str) -> Vec<Sv> {
    let mut out = Vec::new();
    for f in fs {
        match f {
            Feature::AttributeBonus { target, amount, per_level } => {
                out.push(
                    base("attribute_bonus", *amount, *per_level)
                        .with("attribute", Sv::Rec(Rec::code("ref_attribute", &ids::stat_to_ref(target)))),
                );
            }
            Feature::SkillBonus { skills, amount, per_level } => {
                for s in skills {
                    match ctx.find("skill", s) {
                        Some(r) => out.push(base("skill_bonus", *amount, *per_level).with("skill", Sv::Rec(r))),
                        None => ctx.gap("features.skill_bonus: навык не найден, бонус отброшен", format!("{what} -> {s}")),
                    }
                }
            }
            other => ctx.gap(
                &format!("features.{}: нет аналога в схеме, отброшено", other.type_name()),
                what.to_string(),
            ),
        }
    }
    out
}

pub fn from_db(j: &J, report: &mut Report, what: &str) -> Vec<Feature> {
    let mut out: Vec<Feature> = Vec::new();
    for f in jsonx::arr(j, "features") {
        let kind = jsonx::s(f, "kind").unwrap_or("");
        let amount = jsonx::f(f, "amount").unwrap_or(0.0);
        let per_level = jsonx::b(f, "per_level").unwrap_or(false);
        if jsonx::s(f, "op").is_some_and(|o| o != "add") {
            report.gap(Dir::Export, "features.op = set: в листе нет", what);
            continue;
        }
        match kind {
            "attribute_bonus" => match jsonx::code(f, "attribute") {
                Some(a) => out.push(Feature::AttributeBonus {
                    target: ids::ref_to_stat(&a),
                    amount,
                    per_level,
                }),
                None => report.gap(Dir::Export, "features.attribute_bonus без attribute", what),
            },
            "skill_bonus" => {
                let Some(sid) = jsonx::sid_of(f, "skill") else {
                    report.gap(Dir::Export, "features.skill_bonus по категории/специализации: в листе нет", what);
                    continue;
                };
                if f.get("spec").is_some() {
                    report.gap(Dir::Export, "features.skill_bonus.spec: в листе нет", what);
                }
                let id = ids::sid_to_kebab(&sid);
                match out.last_mut() {
                    Some(Feature::SkillBonus {
                        skills,
                        amount: a,
                        per_level: p,
                    }) if *a == amount && *p == per_level => skills.push(id),
                    _ => out.push(Feature::SkillBonus {
                        skills: vec![id],
                        amount,
                        per_level,
                    }),
                }
            }
            k => report.gap(Dir::Export, format!("features.{k}: в листе нет"), what),
        }
    }
    out
}
