//! Проверка дерева пререквизитов (`prereqs`) против персонажа.
use super::Missing;
use super::attributes::Derived;
use super::key;
use super::model::Input;
use super::skills::Skills;
use serde::Serialize;
use serde_json::Value as J;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Violation {
    /// `trait:sid`, `skill:sid`, …
    pub entry: String,
    /// Невыполненные условия (листья дерева).
    pub unmet: Vec<String>,
}

/// (специализация, уровень) изученного навыка.
pub type SpecLevel = (Option<String>, Option<i64>);

/// Факты персонажа, нужные для проверки.
#[derive(Debug, Clone, Default)]
pub struct Ctx {
    pub attrs: BTreeMap<String, f64>,
    pub traits: BTreeMap<String, i64>,
    /// sid -> [(spec, level)]
    pub skills: BTreeMap<String, Vec<SpecLevel>>,
    pub techniques: BTreeMap<String, Option<i64>>,
    /// sid -> школы
    pub spells: BTreeMap<String, Vec<String>>,
    pub magery: Option<i64>,
    pub magery_known: bool,
}

impl Ctx {
    pub fn new(inp: &Input, attrs: &Derived, sk: &Skills, miss: &mut Missing) -> Self {
        let mut c = Ctx {
            attrs: attrs.values.clone(),
            traits: inp
                .traits
                .iter()
                .map(|t| (t.sid.clone(), t.level.unwrap_or(1)))
                .collect(),
            ..Default::default()
        };
        for s in &sk.skills {
            c.skills
                .entry(s.sid.clone())
                .or_default()
                .push((s.spec.clone(), s.level));
        }
        for t in &sk.techniques {
            c.techniques.insert(t.sid.clone(), t.level);
        }
        for s in &inp.spells {
            c.spells.insert(s.sid.clone(), s.colleges.clone());
        }
        match inp.refs.text("magery_trait") {
            Some(m) => {
                c.magery_known = true;
                c.magery = c.traits.get(m).copied();
            }
            None => miss.add("ref_rule:magery_trait"),
        }
        c
    }
}

fn k(n: &J, f: &str) -> String {
    n.get(f).and_then(key).unwrap_or_else(|| "?".into())
}
fn int(n: &J, f: &str) -> Option<i64> {
    n.get(f).and_then(J::as_i64)
}

/// Невыполненные условия узла; пусто = выполнен.
pub fn eval(n: &J, c: &Ctx) -> Vec<String> {
    let t = n.get("type").and_then(J::as_str).unwrap_or("");
    let of = || {
        n.get("of")
            .and_then(J::as_array)
            .cloned()
            .unwrap_or_default()
    };
    match t {
        "and" => of().iter().flat_map(|x| eval(x, c)).collect(),
        "or" => {
            let parts: Vec<Vec<String>> = of().iter().map(|x| eval(x, c)).collect();
            if parts.iter().any(|p| p.is_empty()) {
                vec![]
            } else {
                vec![format!(
                    "одно из: [{}]",
                    parts
                        .iter()
                        .map(|p| p.join(" и "))
                        .collect::<Vec<_>>()
                        .join("; ")
                )]
            }
        }
        "attribute" => {
            let a = k(n, "attribute");
            let min = int(n, "min").unwrap_or(0);
            match c.attrs.get(&a) {
                Some(v) if *v >= min as f64 => vec![],
                v => vec![format!(
                    "{a} >= {min} (есть {})",
                    v.map_or("?".into(), |v| v.to_string())
                )],
            }
        }
        "advantage" => {
            let s = k(n, "trait");
            let need = int(n, "level");
            match (c.traits.get(&s), need) {
                (Some(_), None) => vec![],
                (Some(l), Some(m)) if *l >= m => vec![],
                (l, m) => vec![format!(
                    "черта {s}{} (есть {})",
                    m.map(|m| format!(" ур. {m}")).unwrap_or_default(),
                    l.map_or("нет".into(), |l| format!("ур. {l}"))
                )],
            }
        }
        "skill" => {
            let s = k(n, "skill");
            let spec = n.get("spec").and_then(J::as_str);
            let min = int(n, "min");
            let ok = c.skills.get(&s).is_some_and(|v| {
                v.iter().any(|(sp, l)| {
                    spec.is_none_or(|x| sp.as_deref() == Some(x))
                        && min.is_none_or(|m| l.is_some_and(|l| l >= m))
                })
            });
            if ok {
                vec![]
            } else {
                vec![format!(
                    "навык {s}{}{}",
                    spec.map(|x| format!(" ({x})")).unwrap_or_default(),
                    min.map(|m| format!(" >= {m}")).unwrap_or_default()
                )]
            }
        }
        "technique" => {
            let s = k(n, "technique");
            let min = int(n, "min");
            match c.techniques.get(&s) {
                Some(l) if min.is_none_or(|m| l.is_some_and(|l| l >= m)) => vec![],
                _ => vec![format!(
                    "техника {s}{}",
                    min.map(|m| format!(" >= {m}")).unwrap_or_default()
                )],
            }
        }
        "spell" => {
            let s = k(n, "spell");
            if c.spells.contains_key(&s) {
                vec![]
            } else {
                vec![format!("заклинание {s}")]
            }
        }
        "spells_from_college" => {
            let col = k(n, "college");
            let need = int(n, "count").unwrap_or(1);
            let have = c.spells.values().filter(|cs| cs.contains(&col)).count() as i64;
            if have >= need {
                vec![]
            } else {
                vec![format!("{need} закл. школы {col} (есть {have})")]
            }
        }
        "spells_any" => {
            let need = int(n, "count").unwrap_or(1);
            let have = c.spells.len() as i64;
            if have >= need {
                vec![]
            } else {
                vec![format!("{need} закл. (есть {have})")]
            }
        }
        "magery" => {
            let need = int(n, "level").unwrap_or(0);
            if !c.magery_known {
                return vec!["магия: нет ref_rule:magery_trait".into()];
            }
            match c.magery {
                Some(l) if l >= need => vec![],
                l => vec![format!(
                    "магия ур. {need} (есть {})",
                    l.map_or("нет".into(), |l| l.to_string())
                )],
            }
        }
        other => vec![format!("неизвестный узел {other:?}")],
    }
}

/// Все записи персонажа с невыполненными пререквизитами.
pub fn check_all(inp: &Input, c: &Ctx) -> Vec<Violation> {
    let mut out = Vec::new();
    let mut push = |tb: &str, sid: &str, p: &Option<J>| {
        if let Some(p) = p {
            let unmet = eval(p, c);
            if !unmet.is_empty() {
                out.push(Violation {
                    entry: format!("{tb}:{sid}"),
                    unmet,
                });
            }
        }
    };
    for t in &inp.traits {
        push("trait", &t.sid, &t.prereqs);
    }
    for s in &inp.skills {
        push("skill", &s.sid, &s.prereqs);
    }
    for t in &inp.techniques {
        push("technique", &t.sid, &t.prereqs);
    }
    for s in &inp.spells {
        push("spell", &s.sid, &s.prereqs);
    }
    out
}
