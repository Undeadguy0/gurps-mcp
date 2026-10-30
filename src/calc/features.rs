//! Эффекты (`features`) черт, навыков, техник и надетых предметов, приведённые к сумме бонусов.
use super::key;
use super::model::Input;
use serde_json::Value as J;

/// Один эффект с учётом уровня источника (`per_level`).
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Attribute {
        attr: String,
        amount: f64,
        set: bool,
    },
    Skill {
        skill: String,
        spec: Option<String>,
        amount: f64,
    },
    Defense {
        defenses: Vec<String>,
        amount: f64,
    },
    LiftingSt(f64),
    StrikingSt(f64),
    Spell {
        college: Option<String>,
        spell: Option<String>,
        amount: f64,
    },
    Damage(DamageEffect),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DamageEffect {
    pub attack_mode: Option<String>,
    pub skill: Option<String>,
    pub set: bool,
    pub amount: f64,
    pub per_die: bool,
    pub damage_type: Option<String>,
    pub reach: Option<Vec<i64>>,
}

/// Разбор одного объекта features; неизвестные/нерасчётные виды — None.
pub fn parse(f: &J, level: i64) -> Option<Effect> {
    let per_level = f.get("per_level").and_then(J::as_bool).unwrap_or(false);
    let scale = if per_level { level as f64 } else { 1.0 };
    let amount = f.get("amount").and_then(J::as_f64).unwrap_or(0.0) * scale;
    let set = f.get("op").and_then(J::as_str) == Some("set");
    let k = |n: &str| f.get(n).and_then(key);
    Some(match f.get("kind")?.as_str()? {
        "attribute_bonus" => Effect::Attribute {
            attr: k("attribute")?,
            amount,
            set,
        },
        "skill_bonus" => Effect::Skill {
            skill: k("skill")?,
            spec: f.get("spec").and_then(J::as_str).map(str::to_string),
            amount,
        },
        "defense_bonus" => Effect::Defense {
            defenses: f
                .get("defenses")
                .and_then(J::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|d| d.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
            amount,
        },
        "lifting_st_bonus" => Effect::LiftingSt(amount),
        "striking_st_bonus" => Effect::StrikingSt(amount),
        "spell_bonus" => Effect::Spell {
            college: k("college"),
            spell: k("spell"),
            amount,
        },
        "damage_bonus" => Effect::Damage(DamageEffect {
            attack_mode: k("attack_mode"),
            skill: k("skill"),
            set,
            amount,
            per_die: f.get("per_die").and_then(J::as_bool).unwrap_or(false),
            damage_type: k("damage_type"),
            reach: f
                .get("reach")
                .and_then(J::as_array)
                .map(|a| a.iter().filter_map(J::as_i64).collect()),
        }),
        _ => return None,
    })
}

/// Все действующие эффекты персонажа.
#[derive(Debug, Clone, Default)]
pub struct Effects(pub Vec<Effect>);

impl Effects {
    pub fn collect(inp: &Input) -> Self {
        let mut v = Vec::new();
        for t in &inp.traits {
            let lvl = t.level.unwrap_or(1);
            v.extend(t.features.iter().filter_map(|f| parse(f, lvl)));
        }
        for s in &inp.skills {
            v.extend(s.features.iter().filter_map(|f| parse(f, 1)));
        }
        for t in &inp.techniques {
            v.extend(t.features.iter().filter_map(|f| parse(f, 1)));
        }
        for i in inp.items.iter().filter(|i| i.equipped) {
            v.extend(i.features.iter().filter_map(|f| parse(f, 1)));
        }
        Self(v)
    }

    pub fn attribute(&self, code: &str) -> (f64, Option<f64>) {
        let mut add = 0.0;
        let mut set = None;
        for e in &self.0 {
            if let Effect::Attribute {
                attr,
                amount,
                set: s,
            } = e
                && attr == code
            {
                if *s {
                    set = Some(*amount);
                } else {
                    add += amount;
                }
            }
        }
        (add, set)
    }

    pub fn skill(&self, sid: &str, spec: Option<&str>) -> i64 {
        self.0
            .iter()
            .filter_map(|e| match e {
                Effect::Skill {
                    skill,
                    spec: s,
                    amount,
                } if skill == sid && (s.is_none() || s.as_deref() == spec) => Some(*amount),
                _ => None,
            })
            .sum::<f64>() as i64
    }

    /// Бонус к защите; `defenses` пусто = ко всем.
    pub fn defense(&self, which: &str) -> i64 {
        self.0
            .iter()
            .filter_map(|e| match e {
                Effect::Defense { defenses, amount }
                    if defenses.is_empty() || defenses.iter().any(|d| d == which) =>
                {
                    Some(*amount)
                }
                _ => None,
            })
            .sum::<f64>() as i64
    }

    pub fn lifting_st(&self) -> f64 {
        self.0
            .iter()
            .filter_map(|e| {
                if let Effect::LiftingSt(a) = e {
                    Some(*a)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn striking_st(&self) -> f64 {
        self.0
            .iter()
            .filter_map(|e| {
                if let Effect::StrikingSt(a) = e {
                    Some(*a)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Бонус к заклинанию: общий + по любой из его школ + по самому заклинанию.
    pub fn spell(&self, sid: &str, colleges: &[String]) -> i64 {
        self.0
            .iter()
            .filter_map(|e| match e {
                Effect::Spell {
                    college,
                    spell,
                    amount,
                } => {
                    let hit = match (college, spell) {
                        (None, None) => true,
                        (Some(c), _) => colleges.contains(c),
                        (_, Some(s)) => s == sid,
                    };
                    hit.then_some(*amount)
                }
                _ => None,
            })
            .sum::<f64>() as i64
    }

    /// Эффекты урона для режима атаки (по шаблону или по навыку).
    pub fn damage(&self, attack_mode: &str, skill: Option<&str>) -> Vec<&DamageEffect> {
        self.0
            .iter()
            .filter_map(|e| match e {
                Effect::Damage(d)
                    if d.attack_mode.as_deref() == Some(attack_mode)
                        || (d.skill.is_some() && d.skill.as_deref() == skill) =>
                {
                    Some(d)
                }
                _ => None,
            })
            .collect()
    }
}
