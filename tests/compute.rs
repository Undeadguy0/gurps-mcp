//! compute_character / check_prereqs против surreal memory со схемой и сидами из Data/database.
//! Справочники расчёта — реальные данные книги (05_refs Мастера); каталог и персонаж — заглушки.
//! Ожидания по атрибутам, урону от ST, уровням и очкам выводятся из данных БД, а не зашиты числами.
mod common;
use gurps_mcp::db::Db;
use gurps_mcp::error::ToolError;
use gurps_mcp::tools::CatalogTable;
use gurps_mcp::tools::compute::{
    CheckPrereqsParams, ComputeCharacterParams, check_prereqs, compute_character,
};
use serde_json::Value as J;
use std::collections::BTreeMap;

fn attack<'a>(r: &'a J, sid: &str) -> &'a J {
    r["attacks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["sid"] == sid)
        .unwrap_or_else(|| panic!("нет атаки {sid}: {r}"))
}

async fn q(db: &Db, sql: &str) -> J {
    db.one(sql, vec![]).await.unwrap()
}

fn f(v: &J) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("не число: {v}"))
}

fn i(v: &J) -> i64 {
    v.as_i64().unwrap_or_else(|| panic!("не целое: {v}"))
}

async fn rule(db: &Db, code: &str) -> f64 {
    f(&q(db, &format!("RETURN ref_rule:{code}.value;")).await)
}

/// Итоговые атрибуты test_hero по ref_attribute и купленным уровням (без эффектов черт на атрибуты).
async fn expected_attributes(db: &Db) -> (BTreeMap<String, f64>, i64) {
    let defs = q(
        db,
        "RETURN SELECT code, base_value, base_from.map(|$a| record::id($a)) AS base_from,
            base_divisor, step, cost_per_level FROM ref_attribute;",
    )
    .await;
    let lv = q(
        db,
        "RETURN character:test_hero.attributes.map(|$a| { code: record::id($a.attribute), levels: $a.levels });",
    )
    .await;
    let levels: BTreeMap<String, i64> = lv
        .as_array()
        .unwrap()
        .iter()
        .map(|a| (a["code"].as_str().unwrap().to_string(), i(&a["levels"])))
        .collect();
    let defs = defs.as_array().unwrap();
    let mut out = BTreeMap::new();
    let mut cost = 0;
    while out.len() < defs.len() {
        for d in defs {
            let code = d["code"].as_str().unwrap().to_string();
            let from: Vec<String> = d["base_from"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect();
            if out.contains_key(&code) || !from.iter().all(|b| out.contains_key(b)) {
                continue;
            }
            let step = d["step"].as_f64().unwrap_or(1.0);
            let base = if from.is_empty() {
                f(&d["base_value"])
            } else {
                let v = from.iter().map(|b| out[b]).sum::<f64>()
                    / d["base_divisor"].as_f64().unwrap_or(1.0);
                if step.fract() == 0.0 { v.floor() } else { v }
            };
            let l = levels.get(&code).copied().unwrap_or(0);
            cost += l * i(&d["cost_per_level"]);
            out.insert(code, base + l as f64 * step);
        }
    }
    (out, cost)
}

#[tokio::test]
async fn compute_against_seeded_db() {
    let Some((_s, db)) = common::start().await else {
        return;
    };
    let r = compute_character(
        &db,
        ComputeCharacterParams {
            id: "test_hero".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(r["missing_constants"], serde_json::json!([]), "{r}");

    // атрибуты: база и шаги — из ref_attribute, уровни — из персонажа
    let (attrs, attr_cost) = expected_attributes(&db).await;
    let v = &r["attributes"]["values"];
    for (code, want) in &attrs {
        assert_eq!(f(&v[code]), *want, "{code}: {r}");
    }
    let st = attrs["ST"];

    // Basic Lift = ST^2 / делитель, округление с порога (ref_rule)
    let bl = st * st / rule(&db, "basic_lift_divisor").await;
    let bl = if bl >= rule(&db, "basic_lift_round_min").await {
        bl.round()
    } else {
        bl
    };
    assert_eq!(f(&r["attributes"]["basic_lift"]), bl, "{r}");

    // ударная ST = ST + бонусы striking_st_bonus черт персонажа
    let bonus = i(&q(
        &db,
        "RETURN math::sum((SELECT level, out.features AS fx FROM has_trait WHERE in = character:test_hero)
            .map(|$t| math::sum(($t.fx ?? []).filter(|$x| $x.kind = 'striking_st_bonus')
                .map(|$x| $x.amount * IF $x.per_level { $t.level ?? 1 } ELSE { 1 }))));",
    )
    .await);
    let striking = st + bonus as f64;
    assert_eq!(f(&r["attributes"]["striking_st"]), striking, "{r}");

    // thr/sw — строка ref_st_damage для ударной ST
    let row = q(
        &db,
        &format!("RETURN (SELECT thr, sw FROM ref_st_damage WHERE st = {striking})[0];"),
    )
    .await;
    assert_eq!(r["attributes"]["thrust"], row["thr"], "{r}");
    assert_eq!(r["attributes"]["swing"], row["sw"], "{r}");

    assert_eq!(
        r["attributes"]["current_hp"],
        q(&db, "RETURN character:test_hero.current_hp;").await,
        "{r}"
    );
    assert_eq!(r["attributes"]["defense_bonus"]["parry"], 2, "{r}");

    // удар рукой: thr + мод атаки + бонус навыка за кубик; когти меняют тип на cut
    let punch = attack(&r, "punch");
    let pmod = i(&q(&db, "RETURN attack_mode:['basic', 'punch'].damage.mod;").await);
    let per_die = i(&q(
        &db,
        "RETURN skill:['basic', 'unarmed_a'].features.filter(|$x| $x.kind = 'damage_bonus' AND $x.per_die)[0].amount;",
    )
    .await);
    let dice = i(&row["thr"]["dice"]);
    assert_eq!(punch["damage"]["dice"], dice, "{punch}");
    assert_eq!(
        i(&punch["damage"]["mod"]),
        i(&row["thr"]["mod"]) + pmod + per_die * dice,
        "{punch}"
    );
    assert_eq!(punch["damage_type"], "cut", "{punch}");

    // уровень безоружного навыка: DX + base_mod(E) + смещение за 1 очко
    let unarmed = attrs["DX"] as i64
        + i(&q(&db, "RETURN ref_difficulty:E.base_mod;").await)
        + i(&q(
            &db,
            "RETURN (SELECT VALUE level_offset FROM ref_skill_cost WHERE points = 1)[0];",
        )
        .await);
    assert_eq!(i(&punch["level"]), unarmed, "{punch}");
    let parry = rule(&db, "parry_base").await as i64
        + (unarmed as f64 / rule(&db, "parry_skill_divisor").await).floor() as i64
        + 2;
    assert_eq!(i(&punch["parry"]), parry, "{punch}");
    let kick_mod = i(&q(&db, "RETURN attack_mode:['basic', 'kick'].skill_modifier;").await);
    assert_eq!(i(&attack(&r, "kick")["level"]), unarmed + kick_mod, "{r}");

    // заклинание: IQ + base_mod(H) + смещение за 1 очко + Magery 1
    let sp = &r["spells"][0];
    let spell = attrs["IQ"] as i64
        + i(&q(&db, "RETURN ref_difficulty:H.base_mod;").await)
        + i(&q(
            &db,
            "RETURN (SELECT VALUE level_offset FROM ref_skill_cost WHERE points = 1)[0];",
        )
        .await)
        + 1;
    assert_eq!(i(&sp["level"]), spell, "{r}");

    // очки
    let p = &r["points"];
    assert_eq!(i(&p["attributes"]), attr_cost, "{r}");
    assert_eq!(
        p["skills"],
        q(
            &db,
            "RETURN math::sum(SELECT VALUE points FROM has_skill WHERE in = character:test_hero);"
        )
        .await,
        "{r}"
    );
    assert_eq!(
        i(&p["ledger_spent"]),
        -i(&q(&db, "RETURN math::sum(SELECT VALUE delta FROM point_ledger WHERE character = character:test_hero AND kind = 'spend');").await),
        "{r}"
    );
    assert_eq!(p["matches_ledger"], false, "{r}");

    // check_prereqs: spell_b требует магию 2 — не выполнено
    let c = check_prereqs(
        &db,
        CheckPrereqsParams {
            character: "test_hero".into(),
            table: CatalogTable::Spell,
            sid: "spell_b".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(c["ok"], false, "{c}");
    assert!(c["unmet"].to_string().contains("магия"), "{c}");
    // spell_a: магия 1 — выполнено
    let c = check_prereqs(
        &db,
        CheckPrereqsParams {
            character: "test_hero".into(),
            table: CatalogTable::Spell,
            sid: "spell_a".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(c["ok"], true, "{c}");
    // ошибки
    let e = compute_character(
        &db,
        ComputeCharacterParams {
            id: "nobody".into(),
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(e, ToolError::NotFound(_)));
    let e = check_prereqs(
        &db,
        CheckPrereqsParams {
            character: "test_hero".into(),
            table: CatalogTable::Skill,
            sid: "nope".into(),
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(e, ToolError::NotFound(_)));
}

/// Урон от ST вне таблицы: правила ref_rule из БД (между строками и выше последней строки).
#[tokio::test]
async fn st_damage_rules_from_db() {
    let Some((_s, db)) = common::start().await else {
        return;
    };
    let refs: gurps_mcp::calc::model::Refs = serde_json::from_value(
        db.one(
            &format!("RETURN {};", gurps_mcp::calc::loader::REFS),
            vec![],
        )
        .await
        .unwrap(),
    )
    .unwrap();
    let top = refs.st_damage.iter().max_by_key(|r| r.st).unwrap();
    let step = rule(&db, "st_damage_extra_st").await;
    let per = rule(&db, "st_damage_extra_dice").await as i64;
    let mut m = gurps_mcp::calc::Missing::default();
    let st = top.st + 2 * step as i64 + 1;
    let (thr, sw) = gurps_mcp::calc::st_damage::lookup(&refs, st, &mut m).unwrap();
    assert_eq!(thr.dice, top.thr.dice + 2 * per);
    assert_eq!(sw.dice, top.sw.dice + 2 * per);
    assert_eq!((thr.add, sw.add), (top.thr.add, top.sw.add));
    // между строками таблицы (есть пропуски, например 41..44): строка снизу
    let mut sts: Vec<i64> = refs.st_damage.iter().map(|r| r.st).collect();
    sts.sort();
    let gap = sts
        .windows(2)
        .find(|w| w[1] - w[0] > 1)
        .expect("в таблице нет пропусков");
    let lower = refs.st_damage.iter().find(|r| r.st == gap[0]).unwrap();
    let got = gurps_mcp::calc::st_damage::lookup(&refs, gap[0] + 1, &mut m).unwrap();
    assert_eq!(got, (lower.thr, lower.sw));
    assert!(m.0.is_empty(), "{m:?}");
}
