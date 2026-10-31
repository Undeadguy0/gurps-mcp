//! Модуль calc: чистые функции на синтетических данных (числа фиктивные, не GURPS).
use gurps_mcp::calc::{self, Missing, key, model::*};
use serde_json::json;

fn refs() -> Refs {
    serde_json::from_value(json!({
        "attributes": [
            { "code": "ST", "kind": "basic", "base_value": 10, "cost_per_level": 10 },
            { "code": "DX", "kind": "basic", "base_value": 10, "cost_per_level": 20 },
            { "code": "HT", "kind": "basic", "base_value": 10, "cost_per_level": 10 },
            { "code": "IQ", "kind": "basic", "base_value": 10, "cost_per_level": 20 },
            { "code": "HP", "kind": "secondary", "base_from": ["ST"], "cost_per_level": 2 },
            { "code": "Speed", "kind": "secondary", "base_from": ["DX", "HT"], "base_divisor": 4, "step": 0.25, "cost_per_level": 5 },
            { "code": "Move", "kind": "secondary", "base_from": ["Speed"], "cost_per_level": 5 }
        ],
        "difficulties": [
            { "code": "E", "base_mod": 0 }, { "code": "A", "base_mod": -1, "technique_costs": [2, 1] }
        ],
        "skill_costs": [
            { "points": 1, "level_offset": 0 }, { "points": 2, "level_offset": 1 }, { "points": 4, "level_offset": 2 }
        ],
        "st_damage": [ { "st": 12, "thr": { "dice": 1, "mod": -1 }, "sw": { "dice": 1, "mod": 2 } } ],
        "rules": [
            { "code": "basic_lift_divisor", "value": 5 }, { "code": "basic_lift_round_min", "value": 10 },
            { "code": "parry_base", "value": 3 }, { "code": "parry_skill_divisor", "value": 2 },
            { "code": "limitation_floor_percent", "value": -80 }, { "code": "skill_cost_extra_points", "value": 4 },
            { "code": "spell_attribute", "text": "IQ" }, { "code": "magery_trait", "text": "magery" }
        ]
    }))
    .unwrap()
}

#[test]
fn key_of_record_strings() {
    assert_eq!(
        key(&json!("skill:['basic', 'sword_a']")).as_deref(),
        Some("sword_a")
    );
    assert_eq!(key(&json!("ref_attribute:DX")).as_deref(), Some("DX"));
    assert_eq!(key(&json!(1)), None);
}

#[test]
fn skill_points_table_and_extra() {
    let r = refs();
    let mut m = Missing::default();
    assert_eq!(calc::skills::offset_for_points(&r, 0, &mut m), None);
    assert_eq!(calc::skills::offset_for_points(&r, 1, &mut m), Some(0));
    assert_eq!(calc::skills::offset_for_points(&r, 3, &mut m), Some(1));
    assert_eq!(calc::skills::offset_for_points(&r, 4, &mut m), Some(2));
    assert_eq!(calc::skills::offset_for_points(&r, 12, &mut m), Some(4));
    assert_eq!(calc::skills::relative_for(&r, "A", 4, &mut m), Some(1));
    assert!(m.0.is_empty());
    assert_eq!(calc::skills::relative_for(&r, "VH", 4, &mut m), None);
    assert!(m.0.contains("ref_difficulty:VH.base_mod"));
}

#[test]
fn technique_steps_repeat_last_cost() {
    assert_eq!(calc::skills::technique_steps(&[2, 1], 0), 0);
    assert_eq!(calc::skills::technique_steps(&[2, 1], 1), 0);
    assert_eq!(calc::skills::technique_steps(&[2, 1], 2), 1);
    assert_eq!(calc::skills::technique_steps(&[2, 1], 5), 4);
    assert_eq!(calc::skills::technique_steps(&[], 5), 0);
}

fn hero() -> Input {
    serde_json::from_value(json!({
        "attributes": [ { "code": "ST", "levels": 2 }, { "code": "Speed", "levels": 1 } ],
        "traits": [
            { "sid": "claws", "cost": { "type": "fixed", "base": 5 },
              "features": [ { "kind": "damage_bonus", "op": "set", "attack_mode": "attack_mode:['b', 'punch']",
                              "damage_type": "ref_damage_type:cut" } ] },
            { "sid": "magery", "level": 2, "cost": { "type": "base_plus_level", "base": 5, "per_level": 10 },
              "modifiers": [ { "type": "lim", "value_kind": "%", "amount": -90 } ],
              "features": [ { "kind": "spell_bonus", "amount": 1, "per_level": true } ] },
            { "sid": "brawny", "level": 2, "cost": { "type": "per_level", "per_level": 3 },
              "features": [ { "kind": "lifting_st_bonus", "amount": 1, "per_level": true },
                            { "kind": "defense_bonus", "amount": 1, "defenses": ["parry"] } ] }
        ],
        "skills": [
            { "sid": "brawl", "attribute": "DX", "difficulty": "E", "points": 4,
              "features": [ { "kind": "damage_bonus", "skill": "skill:['b', 'brawl']", "amount": 1, "per_die": true } ] }
        ],
        "spells": [ { "sid": "light", "colleges": ["c1"], "difficulty": "A", "points": 1,
                      "prereqs": { "type": "magery", "level": 3 } } ],
        "techniques": [ { "sid": "elbow", "parent_skill": "brawl", "difficulty": "A", "default_mod": -2,
                          "max_mod": 0, "points": 5 } ],
        "defaults": [ { "skill": "kick_skill", "attribute": "DX", "mod": -2 },
                      { "skill": "kick_skill", "target": "brawl", "mod": -3 } ],
        "catalog_skills": [ { "sid": "kick_skill", "attribute": "DX" } ],
        "natural_attacks": [
            { "sid": "punch", "natural": true, "skill": "brawl", "damage": { "base": "thr", "mod": -1, "type": "cr" },
              "reach": [0], "parry": 0, "skill_damage_bonus": { "min_relative": 3, "per_die": 1 } },
            { "sid": "kick", "natural": true, "skill": "kick_skill", "damage": { "base": "thr", "type": "cr" },
              "skill_modifier": -1, "technique": "elbow" }
        ],
        "ledger": [ { "kind": "award", "delta": 100 }, { "kind": "spend", "delta": -50 } ]
    }))
    .map(|mut i: Input| {
        i.refs = enc_refs();
        i
    })
    .unwrap()
}

#[test]
fn full_compute() {
    let r = calc::compute(&hero());
    assert!(
        r.missing_constants.0.is_empty(),
        "{:?}",
        r.missing_constants
    );
    let v = &r.attributes.values;
    assert_eq!(v["ST"], 12.0);
    assert_eq!(v["HP"], 12.0);
    assert_eq!(v["Speed"], 5.25);
    assert_eq!(v["Move"], 5.0, "дробь базы с целым шагом отбрасывается");
    // ST 12 + 2 = 14; 14*14/5 = 39.2 >= 10 -> округление
    assert_eq!(r.attributes.basic_lift, Some(39.0));
    // навык: DX 10 + E(0) + 4 очка(+2) = 12
    let brawl = r.skills.iter().find(|s| s.sid == "brawl").unwrap();
    assert_eq!((brawl.level, brawl.relative), (Some(12), Some(2)));
    // техника: 12 - 2 + 4 шага (2+1+1+1) = 14, но не выше 12 + 0
    assert_eq!(r.techniques[0].level, Some(12));
    // заклинание: IQ 10 + A(-1) + 0 + магия 2
    assert_eq!(r.spells[0].level, Some(11));
    let punch = r.attacks.iter().find(|a| a.sid == "punch").unwrap();
    // thr 1d-1, mod -1 -> 1d-2; +1 за кубик (навык) последним -> 1d-1; относительный 2 < 3 — без skill_damage_bonus
    assert_eq!(punch.damage_text.as_deref(), Some("1d-1"));
    assert_eq!(punch.damage_type.as_deref(), Some("cut"));
    // парирование: 3 + 12/2 + 0 + 1
    assert_eq!(punch.parry, Some(10));
    // пинок: лучшее из техники 12 и умолчания (brawl 12 - 3) -> 12, затем -1
    let kick = r.attacks.iter().find(|a| a.sid == "kick").unwrap();
    assert_eq!(kick.level, Some(11));
    // очки: ST 20 + Speed 5; когти 5; магия (5+20)*(1-0.8)=5; brawny 6; навыки 4; техника 5; закл. 1
    assert_eq!(r.points.attributes, 25);
    assert_eq!(r.points.advantages, 16);
    assert_eq!(r.points.total, 51);
    assert_eq!(r.points.ledger_spent, 50);
    assert!(!r.points.matches_ledger);
    // магия 2 < 3
    assert_eq!(r.prereq_violations.len(), 1);
    assert_eq!(r.prereq_violations[0].entry, "spell:light");
}

#[test]
fn skill_damage_bonus_when_relative_high() {
    let mut h = hero();
    h.skills[0].points = 12; // +4 -> relative 4 >= 3
    let r = calc::compute(&h);
    let punch = r.attacks.iter().find(|a| a.sid == "punch").unwrap();
    // 1d-2 + (1 + 1) за кубик = 1d+0
    assert_eq!(punch.damage_text.as_deref(), Some("1d"));
}

#[test]
fn prereq_tree_or_and() {
    let inp = hero();
    let mut m = Missing::default();
    let fx = calc::features::Effects::collect(&inp);
    let a = calc::attributes::compute(&inp, &fx, &mut m);
    let sk = calc::skills::Skills::compute(&inp, &a, &fx, &mut m);
    let ctx = calc::prereq::Ctx::new(&inp, &a, &sk, &mut m);
    let ok = json!({ "type": "or", "of": [
        { "type": "attribute", "attribute": "ref_attribute:ST", "min": 20 },
        { "type": "skill", "skill": "skill:['b', 'brawl']", "min": 12 } ] });
    assert!(calc::prereq::eval(&ok, &ctx).is_empty());
    let bad = json!({ "type": "and", "of": [
        { "type": "advantage", "trait": "trait:['b', 'magery']", "level": 3 },
        { "type": "spells_from_college", "college": "ref_college:c1", "count": 1 },
        { "type": "spells_any", "count": 2 } ] });
    let u = calc::prereq::eval(&bad, &ctx);
    assert_eq!(u.len(), 2, "{u:?}");
}

#[test]
fn missing_constants_reported() {
    let mut h = hero();
    h.refs.rules.clear();
    h.refs.st_damage.clear();
    let r = calc::compute(&h);
    let m = &r.missing_constants.0;
    for c in [
        "ref_rule:basic_lift_divisor",
        "ref_rule:parry_base",
        "ref_rule:spell_attribute",
        "ref_rule:magery_trait",
        "ref_rule:limitation_floor_percent",
        "ref_st_damage для ST 12",
    ] {
        assert!(m.contains(c), "{c}: {m:?}");
    }
    assert_eq!(r.attributes.thrust, None);
}

fn enc_refs() -> Refs {
    let mut r = refs();
    let more: Refs = serde_json::from_value(json!({
        "rules": [
            { "code": "dodge_attribute", "text": "Speed" }, { "code": "dodge_base", "value": 3 },
            { "code": "dodge_min", "value": 1 }, { "code": "encumbrance_move_min", "value": 1 },
            { "code": "move_attribute", "text": "Move" },
            { "code": "trait_cost_negative_rounding", "text": "toward_positive" }
        ],
        "encumbrance": [
            { "code": "heavy", "level": 2, "max_load_bl": 6, "move_mult": 0.4, "dodge_mod": -2 },
            { "code": "none", "level": 0, "max_load_bl": 1, "move_mult": 1, "dodge_mod": 0 },
            { "code": "light", "level": 1, "max_load_bl": 2, "move_mult": 0.8, "dodge_mod": -1 }
        ]
    }))
    .unwrap();
    r.rules.extend(more.rules);
    r.encumbrance = more.encumbrance;
    r
}

#[test]
fn encumbrance_levels_inclusive_and_overload() {
    let r = enc_refs();
    let lv = |w| calc::encumbrance::level_for(&r, w, 20.0).map(|(x, o)| (x.level, o));
    assert_eq!(lv(0.0), Some((0, false)));
    assert_eq!(lv(20.0), Some((0, false)));
    assert_eq!(lv(20.5), Some((1, false)));
    assert_eq!(lv(120.0), Some((2, false)));
    assert_eq!(lv(120.5), Some((2, true)));
}

#[test]
fn encumbrance_move_and_dodge_minimums() {
    let r = enc_refs();
    let mut m = Missing::default();
    let heavy = &r.encumbrance[0];
    assert_eq!(calc::encumbrance::move_for(&r, 7.0, heavy, &mut m), 2);
    assert_eq!(
        calc::encumbrance::move_for(&r, 2.0, heavy, &mut m),
        1,
        "минимум"
    );
    assert_eq!(calc::encumbrance::move_for(&r, 0.0, heavy, &mut m), 0);
    assert_eq!(calc::encumbrance::dodge_for(&r, 8, heavy, &mut m), 6);
    assert_eq!(
        calc::encumbrance::dodge_for(&r, 2, heavy, &mut m),
        1,
        "минимум"
    );
    assert!(m.0.is_empty(), "{:?}", m.0);
}

#[test]
fn encumbrance_and_defenses_in_compute() {
    let inp: Input = serde_json::from_value(json!({ "carried_weight": 30 })).unwrap();
    let inp = Input {
        refs: enc_refs(),
        ..inp
    };
    let c = calc::compute(&inp);
    let e = &c.encumbrance;
    // ST 10: BL 20; Speed 5, Move 5, Dodge 8
    assert_eq!((e.level, e.overloaded), (Some(1), false));
    assert_eq!(
        (e.base_dodge, e.dodge, e.move_value),
        (Some(8), Some(7), Some(4))
    );
    assert_eq!(e.table.len(), 3);
    assert_eq!(e.table[2].max_weight, Some(120.0));
    assert_eq!(c.defenses.dodge, Some(7));
    assert_eq!(c.defenses.parry, None);
}

#[test]
fn negative_trait_rounding_from_rules_and_overrides() {
    let t: OwnedTrait = serde_json::from_value(json!({
        "sid": "x", "cost": { "type": "fixed", "base": -15 },
        "modifiers": [ { "type": "lim", "value_kind": "%", "amount": -50 } ]
    }))
    .unwrap();
    let r = enc_refs();
    let mut m = Missing::default();
    assert_eq!(calc::points::trait_cost(&r, &t, &mut m), -7);
    let over: Vec<RuleRow> = serde_json::from_value(
        json!([{ "code": "trait_cost_negative_rounding", "text": "by_magnitude" }]),
    )
    .unwrap();
    assert_eq!(
        calc::points::trait_cost(&r.with_overrides(&over), &t, &mut m),
        -8
    );
    assert!(m.0.is_empty(), "{:?}", m.0);
    assert_eq!(calc::points::trait_cost(&refs(), &t, &mut m), -7);
    assert!(m.0.contains("ref_rule:trait_cost_negative_rounding"));
}

/// Значения block_* — только тестовые (в сидах их нет до проверки Мастером).
#[test]
fn block_and_shield_db() {
    let mut r = enc_refs();
    let more: Vec<RuleRow> = serde_json::from_value(json!([
        { "code": "block_base", "value": 3 }, { "code": "block_skill_divisor", "value": 2 },
        { "code": "block_rounding", "text": "floor" }
    ]))
    .unwrap();
    r.rules.extend(more);
    let mut inp: Input = serde_json::from_value(json!({
        "carried_weight": 30,
        "attributes": [ { "code": "ST", "levels": 2 } ],
        "skills": [ { "sid": "shield", "attribute": "DX", "difficulty": "E", "points": 4, "spec": "buckler" } ],
        "traits": [ { "sid": "cr", "cost": { "type": "fixed", "base": 15 },
                      "features": [ { "kind": "defense_bonus", "amount": 1 } ] } ],
        "items": [
            { "sid": "buckler", "equipped": true, "shield": { "db": 1, "block_skill": "shield", "block_spec": "buckler" } },
            { "sid": "big", "equipped": false, "shield": { "db": 3, "block_skill": "shield" } }
        ]
    }))
    .unwrap();
    inp.refs = r.clone();
    let c = calc::compute(&inp);
    assert!(
        c.missing_constants.0.is_empty(),
        "{:?}",
        c.missing_constants
    );
    let d = &c.defenses;
    // ST 12: BL 29, перенос 30 — Light
    // Shield (E) 4 очка: DX+2 = 12 -> 3 + 6 + 1 (бонус) + 1 (DB) = 11; нагрузка Light на Block не влияет
    assert_eq!(
        (d.block, d.block_item.as_deref(), d.db),
        (Some(11), Some("buckler"), 1)
    );
    // Dodge: 8 - 1 (Light) + 1 (бонус) + 1 (DB) = 9
    assert_eq!(d.dodge, Some(9));
    let mut m = Missing::default();
    assert_eq!(calc::encumbrance::block(&enc_refs(), 12, &mut m), None);
    assert!(m.0.contains("ref_rule:block_base") && m.0.contains("ref_rule:block_skill_divisor"));
}
