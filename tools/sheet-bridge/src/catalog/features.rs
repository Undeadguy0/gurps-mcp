//! Особенности (features): лист `Feature` <-> объекты `features[]` схемы.

use super::{spells, Ctx};
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::Feature;
use crate::sql::{Rec, Sv};
use serde_json::Value as J;
use std::collections::BTreeMap;

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
            Feature::LiftingStBonus { amount, per_level } => out.push(base("lifting_st_bonus", *amount, *per_level)),
            Feature::StrikingStBonus { amount, per_level } => out.push(base("striking_st_bonus", *amount, *per_level)),
            Feature::DefenseBonus {
                amount,
                defenses,
                per_level,
            } => {
                let mut o = base("defense_bonus", *amount, *per_level);
                let mut ds: Vec<String> = defenses.iter().map(|d| d.trim().to_lowercase()).collect();
                ds.dedup();
                if !ds.is_empty() {
                    o.set("defenses", Sv::Arr(ds.iter().map(Sv::str).collect()));
                }
                out.push(o);
            }
            Feature::SpellBonus {
                amount,
                colleges,
                per_level,
            } => {
                if colleges.is_empty() {
                    out.push(base("spell_bonus", *amount, *per_level));
                }
                for c in colleges {
                    out.push(base("spell_bonus", *amount, *per_level).with("college", Sv::Rec(spells::college_rec(ctx.resolver, c))));
                }
            }
        }
    }
    out
}

pub fn from_db(j: &J, report: &mut Report, colleges: &BTreeMap<String, String>, what: &str) -> Vec<Feature> {
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
            "lifting_st_bonus" => out.push(Feature::LiftingStBonus { amount, per_level }),
            "striking_st_bonus" => out.push(Feature::StrikingStBonus { amount, per_level }),
            "defense_bonus" => out.push(Feature::DefenseBonus {
                amount,
                defenses: jsonx::arr(f, "defenses")
                    .iter()
                    .filter_map(|d| d.as_str().map(str::to_string))
                    .collect(),
                per_level,
            }),
            "spell_bonus" => {
                if f.get("spell").is_some_and(|v| !v.is_null()) {
                    report.gap(Dir::Export, "features.spell_bonus по заклинанию: в листе нет", what);
                    continue;
                }
                let college = jsonx::code(f, "college").map(|c| {
                    colleges.get(&c).cloned().unwrap_or_else(|| {
                        report.gap(
                            Dir::Export,
                            "features.spell_bonus.college: у ref_college нет имени en, выведен код",
                            what,
                        );
                        c
                    })
                });
                match (college, out.last_mut()) {
                    (
                        Some(c),
                        Some(Feature::SpellBonus {
                            colleges: cs,
                            amount: a,
                            per_level: p,
                        }),
                    ) if !cs.is_empty() && *a == amount && *p == per_level => cs.push(c),
                    (c, _) => out.push(Feature::SpellBonus {
                        amount,
                        colleges: c.into_iter().collect(),
                        per_level,
                    }),
                }
            }
            k => report.gap(Dir::Export, format!("features.{k}: в листе нет"), what),
        }
    }
    out
}
