//! Разбор строк урона/досягаемости/парирования листа в структуры атаки схемы и обратно.

use super::Ctx;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::AttackDef;
use crate::sql::{Rec, Sv};
use serde_json::Value as J;

/// `sw+1` -> (Some(sw), None, 1); `2d-1` -> (None, Some(2), -1).
pub fn parse_damage(s: &str) -> Option<(Option<&'static str>, Option<i64>, i64)> {
    let t = s.trim().replace(' ', "");
    let (head, rest) = match t.find(['+', '-']) {
        Some(i) => (&t[..i], &t[i..]),
        None => (t.as_str(), ""),
    };
    let m: i64 = if rest.is_empty() { 0 } else { rest.parse().ok()? };
    match head {
        "sw" => Some((Some("sw"), None, m)),
        "thr" => Some((Some("thr"), None, m)),
        h => {
            let n: i64 = h.strip_suffix('d')?.parse().ok()?;
            Some((None, Some(n), m))
        }
    }
}

/// `C,1` -> [0, 1]; `1-3` -> [1, 2, 3].
pub fn parse_reach(s: &str) -> Option<Vec<i64>> {
    let mut out = Vec::new();
    for part in s.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let one = |x: &str| -> Option<i64> {
            match x.trim().trim_end_matches('*') {
                "C" | "c" => Some(0),
                n => n.parse().ok(),
            }
        };
        if let Some((a, b)) = part.split_once('-') {
            let (a, b) = (one(a)?, one(b)?);
            out.extend(a..=b);
        } else {
            out.push(one(part)?);
        }
    }
    (!out.is_empty()).then_some(out)
}

/// Поля режима атаки (общие для `item.attack_modes[*]` и `attack_mode`).
pub fn attack_to_db(ctx: &mut Ctx, a: &AttackDef, default_skill: &str, what: &str) -> Sv {
    let mut dmg = Sv::obj()
        .with("raw", Sv::str(&a.damage))
        .with("type", Sv::Rec(Rec::code("ref_damage_type", &a.damage_type)));
    match parse_damage(&a.damage) {
        Some((base, dice, m)) => {
            dmg.set("base", Sv::opt(base, Sv::str));
            dmg.set("dice", Sv::opt(dice, Sv::Int));
            dmg.set("mod", Sv::Int(m));
        }
        None => ctx.gap("attack.damage: не разобран (сохранён только raw)", format!("{what}: {}", a.damage)),
    }
    let mut m = Sv::obj().with("damage", dmg);
    let skill = a.skill.as_deref().unwrap_or(default_skill);
    if !skill.is_empty() {
        match ctx.find("skill", skill) {
            Some(r) => {
                m.set("skill", Sv::Rec(r));
            }
            None => ctx.gap("attack.skill: навык не найден, отброшен", format!("{what} -> {skill}")),
        }
    }
    if !a.reach.trim().is_empty() {
        m.set("reach_raw", Sv::str(&a.reach));
        match parse_reach(&a.reach) {
            Some(r) => {
                m.set("reach", Sv::Arr(r.into_iter().map(Sv::Int).collect()));
            }
            None => ctx.gap(
                "attack.reach: не разобран (сохранён только reach_raw)",
                format!("{what}: {}", a.reach),
            ),
        }
    }
    match a.parry {
        Some(p) => {
            m.set("parry_raw", Sv::str(p.to_string()));
            m.set("parry", Sv::obj().with("value", Sv::Int(p as i64)));
        }
        None => {
            m.set("parry_raw", Sv::str("No"));
        }
    }
    let mut lost = vec![];
    if a.skill_modifier != 0 {
        lost.push("skill_modifier");
    }
    if a.technique.is_some() {
        lost.push("technique");
    }
    if !a.defaults.is_empty() {
        lost.push("defaults");
    }
    if a.skill_damage_bonus.is_some() {
        lost.push("skill_damage_bonus");
    }
    for l in lost {
        ctx.gap(&format!("attack.{l}: в схеме нет, отброшено"), what.to_string());
    }
    m
}

pub fn attack_from_db(m: &J, mode: String, weapon_skill: &str, report: &mut Report, what: &str) -> AttackDef {
    let dmg = m.get("damage").cloned().unwrap_or(J::Null);
    let damage = jsonx::s(&dmg, "raw").map(str::to_string).unwrap_or_else(|| {
        let base = jsonx::s(&dmg, "base").map(str::to_string);
        let dice = jsonx::i(&dmg, "dice").map(|d| format!("{d}d"));
        let md = jsonx::i(&dmg, "mod").unwrap_or(0);
        let head = base.or(dice).unwrap_or_default();
        match md {
            0 => head,
            x if x > 0 => format!("{head}+{x}"),
            x => format!("{head}{x}"),
        }
    });
    if dmg.get("modifiers").is_some_and(|v| !v.is_null()) || dmg.get("armor_divisor").is_some_and(|v| !v.is_null()) {
        report.gap(Dir::Export, "attack.damage.modifiers/armor_divisor: в листе нет", what);
    }
    if m.get("notes").is_some_and(|v| !v.is_null()) {
        report.gap(Dir::Export, "attack.notes: в листе нет", what);
    }
    let reach = jsonx::s(m, "reach_raw").map(str::to_string).unwrap_or_else(|| {
        jsonx::arr(m, "reach")
            .iter()
            .filter_map(|x| x.as_i64())
            .map(|x| if x == 0 { "C".into() } else { x.to_string() })
            .collect::<Vec<_>>()
            .join(",")
    });
    let parry = m
        .get("parry")
        .and_then(|p| jsonx::i(p, "value"))
        .or_else(|| jsonx::s(m, "parry_raw").and_then(|r| r.trim_end_matches(['F', 'U']).parse().ok()))
        .map(|x| x as i32);
    let skill = jsonx::sid_of(m, "skill").map(|s| crate::ids::sid_to_kebab(&s));
    AttackDef {
        mode,
        damage,
        damage_type: jsonx::code(&dmg, "type").unwrap_or_else(|| {
            report.gap(Dir::Export, "attack.damage.type пуст", what);
            String::new()
        }),
        reach,
        parry,
        skill: skill.filter(|s| s != weapon_skill),
        skill_modifier: 0,
        technique: None,
        defaults: vec![],
        skill_damage_bonus: None,
    }
}
