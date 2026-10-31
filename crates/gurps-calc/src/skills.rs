//! Уровни навыков, техник и заклинаний: очки + сложность + атрибут, умолчания (лучшее), эффекты.
use super::Missing;
use super::attributes::Derived;
use super::features::Effects;
use super::model::{Input, Refs};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SkillLevel {
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    pub points: i64,
    /// Итоговый уровень (None — не хватает констант/атрибута).
    pub level: Option<i64>,
    /// Уровень относительно атрибута (для навыков и заклинаний) или навыка-родителя (техники).
    pub relative: Option<i64>,
    /// Откуда уровень: `points` | `default:<attr|skill>` | `technique`.
    pub source: String,
    /// Техники: предельный уровень (родитель + max_mod).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<i64>,
}

impl SkillLevel {
    /// Уровень взят по умолчанию (не куплен очками).
    pub fn is_default(&self) -> bool {
        self.source.starts_with("default")
    }
}

/// Прибавка к уровню за очки по таблице ref_skill_cost (+ правило сверх таблицы).
pub fn offset_for_points(refs: &Refs, points: i64, miss: &mut Missing) -> Option<i64> {
    if points <= 0 {
        return None;
    }
    let mut rows = refs.skill_costs.clone();
    rows.sort_by_key(|r| r.points);
    let Some(last) = rows.last().copied() else {
        miss.add("ref_skill_cost (таблица пуста)");
        return None;
    };
    if points > last.points {
        let Some(extra) = refs.value("skill_cost_extra_points").filter(|e| *e > 0.0) else {
            miss.add("ref_rule:skill_cost_extra_points");
            return None;
        };
        return Some(last.level_offset + ((points - last.points) as f64 / extra).floor() as i64);
    }
    rows.iter()
        .rev()
        .find(|r| r.points <= points)
        .map(|r| r.level_offset)
        .or_else(|| {
            miss.add(format!("ref_skill_cost для {points} очк."));
            None
        })
}

/// Уровень за очки относительно атрибута: base_mod сложности + прибавка за очки.
pub fn relative_for(refs: &Refs, difficulty: &str, points: i64, miss: &mut Missing) -> Option<i64> {
    let base = match refs.diff(difficulty).and_then(|d| d.base_mod) {
        Some(b) => b,
        None => {
            miss.add(format!("ref_difficulty:{difficulty}.base_mod"));
            return None;
        }
    };
    offset_for_points(refs, points, miss).map(|o| base + o)
}

/// Сколько уровней техники над умолчанием куплено за очки (цены шагов; последняя повторяется).
pub fn technique_steps(costs: &[i64], points: i64) -> i64 {
    let (mut spent, mut n) = (0, 0i64);
    let Some(&last) = costs.last() else { return 0 };
    loop {
        let c = costs.get(n as usize).copied().unwrap_or(last);
        if c <= 0 || spent + c > points {
            return n;
        }
        spent += c;
        n += 1;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Skills {
    pub skills: Vec<SkillLevel>,
    pub techniques: Vec<SkillLevel>,
    pub spells: Vec<SkillLevel>,
    /// Уровни навыков каталога, которых нет у персонажа (по умолчанию).
    pub defaulted: Vec<SkillLevel>,
}

impl Skills {
    /// Лучший уровень навыка по sid (изученный или по умолчанию).
    pub fn level(&self, sid: &str) -> Option<i64> {
        self.skills
            .iter()
            .chain(&self.defaulted)
            .filter(|s| s.sid == sid)
            .filter_map(|s| s.level)
            .max()
    }

    pub fn technique(&self, sid: &str) -> Option<i64> {
        self.techniques
            .iter()
            .find(|t| t.sid == sid)
            .and_then(|t| t.level)
    }

    pub fn compute(inp: &Input, attrs: &Derived, fx: &Effects, miss: &mut Missing) -> Self {
        let refs = &inp.refs;
        let attr = |code: &str, miss: &mut Missing| {
            let v = attrs.get(code).map(|v| v.floor() as i64);
            if v.is_none() {
                miss.add(format!("значение атрибута {code}"));
            }
            v
        };
        // 1. изученные за очки
        let mut trained: Vec<(usize, Option<i64>)> = Vec::new();
        for (i, s) in inp.skills.iter().enumerate() {
            let lvl = match (
                attr(&s.attribute, miss),
                relative_for(refs, &s.difficulty, s.points, miss),
            ) {
                (Some(a), Some(r)) => Some(a + r),
                _ => None,
            };
            trained.push((i, lvl));
        }
        let trained_of = |sid: &str| {
            trained
                .iter()
                .filter(|(i, _)| inp.skills[*i].sid == sid)
                .filter_map(|(_, l)| *l)
                .max()
        };
        // 2. умолчания: от атрибута или от изученного навыка (без цепочек)
        let best_default = |sid: &str, miss: &mut Missing| -> Option<(i64, String)> {
            inp.defaults
                .iter()
                .filter(|d| d.skill == sid)
                .filter_map(|d| {
                    if let Some(a) = &d.attribute {
                        attr(a, miss).map(|v| (v + d.modifier, format!("default:{a}")))
                    } else {
                        let t = d.target.as_deref()?;
                        trained_of(t).map(|v| (v + d.modifier, format!("default:{t}")))
                    }
                })
                .max_by_key(|x| x.0)
        };
        let mut out = Skills::default();
        for (i, lvl) in &trained {
            let s = &inp.skills[*i];
            let def = best_default(&s.sid, miss);
            let (base, source) = match (lvl, def) {
                (Some(l), Some((d, src))) if d > *l => (Some(d), src),
                (Some(l), _) => (Some(*l), "points".to_string()),
                (None, Some((d, src))) => (Some(d), src),
                (None, None) => (None, "points".to_string()),
            };
            let level = base.map(|b| b + fx.skill(&s.sid, s.spec.as_deref()));
            out.skills.push(SkillLevel {
                sid: s.sid.clone(),
                spec: s.spec.clone(),
                points: s.points,
                relative: level
                    .zip(attrs.get(&s.attribute))
                    .map(|(l, a)| l - a.floor() as i64),
                level,
                source,
                max_level: None,
            });
        }
        // навыки каталога без очков (для атак и техник): только по умолчанию
        for c in &inp.catalog_skills {
            if inp.skills.iter().any(|s| s.sid == c.sid) {
                continue;
            }
            if let Some((d, src)) = best_default(&c.sid, miss) {
                let level = d + fx.skill(&c.sid, None);
                out.defaulted.push(SkillLevel {
                    sid: c.sid.clone(),
                    spec: None,
                    points: 0,
                    relative: attrs.get(&c.attribute).map(|a| level - a.floor() as i64),
                    level: Some(level),
                    source: src,
                    max_level: None,
                });
            }
        }
        // 3. техники: умолчание от родителя + купленные шаги, не выше max_mod
        for t in &inp.techniques {
            let parent = out.level(&t.parent_skill);
            let costs = refs
                .diff(&t.difficulty)
                .map(|d| d.technique_costs.clone())
                .unwrap_or_default();
            if costs.is_empty() && t.points > 0 {
                miss.add(format!("ref_difficulty:{}.technique_costs", t.difficulty));
            }
            let steps = technique_steps(&costs, t.points);
            let level = parent.map(|p| {
                let l = p + t.default_mod + steps;
                t.max_mod.map_or(l, |m| l.min(p + m))
            });
            out.techniques.push(SkillLevel {
                sid: t.sid.clone(),
                spec: None,
                points: t.points,
                relative: level.zip(parent).map(|(l, p)| l - p),
                level,
                source: "technique".into(),
                max_level: parent.zip(t.max_mod).map(|(p, m)| p + m),
            });
        }
        // 4. заклинания: атрибут из ref_rule:spell_attribute + сложность + очки + spell_bonus
        let spell_attr = refs.text("spell_attribute").map(str::to_string);
        if spell_attr.is_none() && !inp.spells.is_empty() {
            miss.add("ref_rule:spell_attribute");
        }
        for s in &inp.spells {
            let a = spell_attr.as_deref().and_then(|c| attr(c, miss));
            let rel = relative_for(refs, &s.difficulty, s.points, miss);
            let level = a
                .zip(rel)
                .map(|(a, r)| a + r + fx.spell(&s.sid, &s.colleges));
            out.spells.push(SkillLevel {
                sid: s.sid.clone(),
                spec: None,
                points: s.points,
                relative: level.zip(a).map(|(l, a)| l - a),
                level,
                source: "points".into(),
                max_level: None,
            });
        }
        out
    }
}
