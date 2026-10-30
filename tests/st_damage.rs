//! Урон от ST на синтетической таблице (числа фиктивные): точное, между строками, выше таблицы, нехватка правил.
use gurps_mcp::calc::{Missing, model::*, st_damage::lookup};
use serde_json::json;

fn refs(rules: serde_json::Value) -> Refs {
    serde_json::from_value(json!({
        "st_damage": [
            { "st": 10, "thr": { "dice": 1, "mod": -2 }, "sw": { "dice": 1, "mod": 0 } },
            { "st": 20, "thr": { "dice": 2, "mod": -1 }, "sw": { "dice": 3, "mod": 2 } }
        ],
        "rules": rules
    }))
    .unwrap()
}

fn d(dice: i64, add: i64) -> Dice {
    Dice { dice, add }
}

fn full() -> Refs {
    refs(json!([
        { "code": "st_damage_lookup", "text": "floor" },
        { "code": "st_damage_extra_st", "value": 5 },
        { "code": "st_damage_extra_dice", "value": 2 }
    ]))
}

#[test]
fn exact_between_above() {
    let r = full();
    let mut m = Missing::default();
    assert_eq!(lookup(&r, 10, &mut m), Some((d(1, -2), d(1, 0))));
    assert_eq!(lookup(&r, 15, &mut m), Some((d(1, -2), d(1, 0))));
    assert_eq!(lookup(&r, 24, &mut m), Some((d(2, -1), d(3, 2))));
    assert_eq!(lookup(&r, 25, &mut m), Some((d(4, -1), d(5, 2))));
    assert_eq!(lookup(&r, 31, &mut m), Some((d(6, -1), d(7, 2))));
    assert!(m.0.is_empty(), "{m:?}");
}

#[test]
fn missing_rules_reported() {
    let r = refs(json!([]));
    let mut m = Missing::default();
    assert_eq!(lookup(&r, 15, &mut m), None);
    assert_eq!(lookup(&r, 30, &mut m), None);
    assert_eq!(lookup(&r, 5, &mut m), None);
    for c in [
        "ref_rule:st_damage_lookup",
        "ref_rule:st_damage_extra_st",
        "ref_rule:st_damage_extra_dice",
        "ref_st_damage для ST 5",
    ] {
        assert!(m.0.contains(c), "{c}: {m:?}");
    }
}

#[test]
fn unknown_lookup_mode_reported() {
    let r = refs(json!([{ "code": "st_damage_lookup", "text": "round" }]));
    let mut m = Missing::default();
    assert_eq!(lookup(&r, 15, &mut m), None);
    assert!(m.0.iter().any(|x| x.contains("неизвестный режим")), "{m:?}");
}
