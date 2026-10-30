//! Навыки и техники: `SkillDef`/`TechniqueDef` <-> `skill`/`technique` (+ attr_default, defaults_to).

use super::{Ctx, Entity, Tr};
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::{Difficulty, SkillDef, SkillDefault, TechniqueDef};
use crate::sql::{Rec, Sv};
use serde_json::Value as J;

pub fn diff_rec(d: Difficulty) -> Sv {
    Sv::Rec(Rec::code("ref_difficulty", d.key()))
}

pub fn diff_of(j: &J, report: &mut Report, what: &str) -> Difficulty {
    let c = jsonx::code(j, "difficulty").unwrap_or_default();
    Difficulty::parse(&c).unwrap_or_else(|| {
        report.gap(Dir::Export, "difficulty: неизвестный код, взято A", format!("{what} ({c})"));
        Difficulty::Average
    })
}

pub fn attr_rec(stat: &str) -> Sv {
    Sv::Rec(Rec::code("ref_attribute", &ids::stat_to_ref(stat)))
}

pub fn spec_sid(name: &str) -> String {
    ids::slug(name)
}

pub fn to_db(ctx: &mut Ctx, d: &SkillDef) -> anyhow::Result<Entity> {
    let rec = ctx.rec("skill", &d.id)?;
    let what = format!("skill {}", d.id);
    let spec_mode = if d.specialization_required {
        "required"
    } else if !d.specializations.is_empty() {
        "optional"
    } else {
        "none"
    };
    let specs: Vec<String> = d.specializations.iter().map(|s| spec_sid(s)).collect();
    let mut c = Sv::obj()
        .with("attribute", attr_rec(&d.attribute))
        .with("difficulty", diff_rec(d.difficulty))
        .with("spec_mode", Sv::str(spec_mode))
        .with("specializations", Sv::Arr(specs.iter().cloned().map(Sv::Str).collect()));
    super::set_page(ctx, &mut c, d.page.as_deref(), &what);
    let mut e = Entity::new(rec, c);
    e.tr = ctx.texts("skill", &d.id, &d.name, d.description.as_ref());
    for (sid, name) in specs.iter().zip(&d.specializations) {
        e.tr.push(Tr {
            lang: "en".into(),
            field: format!("spec.{sid}"),
            text: name.clone(),
        });
    }
    for df in &d.defaults {
        match df {
            SkillDefault::Attribute { attribute, modifier } => {
                let a = Rec::code("ref_attribute", &ids::stat_to_ref(attribute));
                if e.attr_defaults.iter().any(|(x, _)| *x == a) {
                    ctx.gap("skill.defaults: повтор атрибута (UNIQUE skill+attribute), отброшен", what.clone());
                } else {
                    e.attr_defaults.push((a, *modifier as i64));
                }
            }
            SkillDefault::Skill { skill, modifier } => match ctx.find("skill", skill) {
                Some(r) => e.defaults_to.push((r, *modifier as i64)),
                None => ctx.gap(
                    "skill.defaults: навык-цель не найден, умолчание отброшено",
                    format!("{what} -> {skill}"),
                ),
            },
        }
    }
    Ok(e)
}

pub fn defaults_from(j: &J) -> Vec<SkillDefault> {
    let mut out = Vec::new();
    for a in jsonx::arr(j, "_attr_defaults") {
        if let Some(c) = jsonx::code(a, "attribute") {
            out.push(SkillDefault::Attribute {
                attribute: ids::ref_to_stat(&c),
                modifier: jsonx::i(a, "mod").unwrap_or(0) as i32,
            });
        }
    }
    for d in jsonx::arr(j, "_defaults_to") {
        if let Some(s) = jsonx::sid_of(d, "out") {
            out.push(SkillDefault::Skill {
                skill: ids::sid_to_kebab(&s),
                modifier: jsonx::i(d, "mod").unwrap_or(0) as i32,
            });
        }
    }
    out
}

pub fn from_db(j: &J, report: &mut Report) -> SkillDef {
    let id = super::kebab_sid(j);
    let what = format!("skill {id}");
    let mode = jsonx::s(j, "spec_mode").unwrap_or("none");
    if mode == "tl" {
        report.gap(Dir::Export, "skill.spec_mode = tl: в листе нет", what.clone());
    }
    for k in ["tl_dependent", "encumbrance_penalty"] {
        if jsonx::b(j, k) == Some(true) {
            report.gap(Dir::Export, format!("skill.{k}: в листе нет"), what.clone());
        }
    }
    if !jsonx::arr(j, "categories").is_empty() {
        report.gap(Dir::Export, "skill.categories: в листе нет", what.clone());
    }
    if j.get("prereqs").is_some_and(|v| !v.is_null()) {
        report.gap(Dir::Export, "skill.prereqs: в листе нет", what.clone());
    }
    if !jsonx::arr(j, "features").is_empty() {
        report.gap(Dir::Export, "skill.features: в листе нет", what.clone());
    }
    for d in jsonx::arr(j, "_defaults_to") {
        if jsonx::b(d, "any_spec") != Some(true) || d.get("spec").is_some() || d.get("cap").is_some() || d.get("from_spec").is_some() {
            report.gap(Dir::Export, "defaults_to.spec/from_spec/cap: в листе нет", what.clone());
        }
    }
    let specializations = jsonx::arr(j, "specializations")
        .iter()
        .filter_map(|s| s.as_str())
        .map(|s| super::tr_get(j, "en", &format!("spec.{s}")).unwrap_or_else(|| s.to_string()))
        .collect();
    SkillDef {
        name: super::name_of(j, report, &what),
        attribute: jsonx::code(j, "attribute")
            .map(|c| ids::ref_to_stat(&c))
            .unwrap_or_else(|| "DX".into()),
        difficulty: diff_of(j, report, &what),
        defaults: defaults_from(j),
        specialization_required: mode == "required",
        specializations,
        page: super::page_of(j),
        description: super::desc_of(j),
        id,
    }
}

pub fn tech_to_db(ctx: &mut Ctx, d: &TechniqueDef) -> anyhow::Result<Entity> {
    let rec = ctx.rec("technique", &d.id)?;
    let what = format!("technique {}", d.id);
    let parent = ctx
        .find("skill", &d.skill)
        .ok_or_else(|| anyhow::anyhow!("{what}: родительский навык {:?} не найден ни в импорте, ни в БД", d.skill))?;
    let mut c = Sv::obj()
        .with("parent_skill", Sv::Rec(parent))
        .with("difficulty", diff_rec(d.difficulty))
        .with("default_mod", Sv::Int(d.modifier as i64))
        .with("max_mod", Sv::Int(d.max as i64));
    super::set_page(ctx, &mut c, d.page.as_deref(), &what);
    let mut e = Entity::new(rec, c);
    e.tr = ctx.texts("technique", &d.id, &d.name, d.description.as_ref());
    Ok(e)
}

pub fn tech_from_db(j: &J, report: &mut Report) -> TechniqueDef {
    let id = super::kebab_sid(j);
    let what = format!("technique {id}");
    for k in ["parent_spec", "max_raw", "derived_from", "attack_delta", "prereqs"] {
        if j.get(k).is_some_and(|v| !v.is_null()) {
            report.gap(Dir::Export, format!("technique.{k}: в листе нет"), what.clone());
        }
    }
    if !jsonx::arr(j, "features").is_empty() || !jsonx::arr(j, "_defaults_to").is_empty() {
        report.gap(Dir::Export, "technique.features/defaults_to: в листе нет", what.clone());
    }
    TechniqueDef {
        name: super::name_of(j, report, &what),
        skill: jsonx::sid_of(j, "parent_skill").map(|s| ids::sid_to_kebab(&s)).unwrap_or_default(),
        difficulty: diff_of(j, report, &what),
        modifier: jsonx::i(j, "default_mod").unwrap_or(0) as i32,
        max: jsonx::i(j, "max_mod").unwrap_or(0) as i32,
        page: super::page_of(j),
        description: super::desc_of(j),
        id,
    }
}
