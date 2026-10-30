//! compute_character / check_prereqs против surreal memory со схемой и сидами-заглушками.
//! Числа — из фиктивных констант сидов (не GURPS): проверяется связка БД -> расчёт.
mod common;
use gurps_mcp::error::ToolError;
use gurps_mcp::tools::CatalogTable;
use gurps_mcp::tools::compute::{
    CheckPrereqsParams, ComputeCharacterParams, check_prereqs, compute_character,
};
use serde_json::Value as J;

fn attack<'a>(r: &'a J, sid: &str) -> &'a J {
    r["attacks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["sid"] == sid)
        .unwrap_or_else(|| panic!("нет атаки {sid}: {r}"))
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
    let v = &r["attributes"]["values"];
    // база 5 (заглушка) + купленные уровни; Speed = (DX + HT) / 2 + 1 шаг 0.25
    assert_eq!(v["ST"], 6.0, "{r}");
    assert_eq!(v["DX"], 7.0, "{r}");
    assert_eq!(v["HP"], 6.0, "{r}");
    assert_eq!(v["Speed"], 6.25, "{r}");
    assert_eq!(v["Move"], 6.0, "{r}");
    assert_eq!(r["attributes"]["striking_st"], 8.0, "{r}");
    assert_eq!(r["attributes"]["basic_lift"], 36.0, "{r}");
    assert_eq!(r["attributes"]["current_hp"], 4, "{r}");
    assert_eq!(r["attributes"]["defense_bonus"]["parry"], 2, "{r}");
    // когти: удар рукой режущий; +1 за кубик от навыка
    let punch = attack(&r, "punch");
    assert_eq!(punch["damage_type"], "cut", "{punch}");
    assert_eq!(punch["damage_text"], "1d+1", "{punch}");
    assert_eq!(punch["level"], 7, "{punch}");
    assert_eq!(punch["parry"], 10, "{punch}");
    // пинок: лучшее из умолчаний (sword_a - 1 = 9) + skill_modifier -1
    assert_eq!(attack(&r, "kick")["level"], 8, "{r}");
    // заклинание: IQ + магия (spell_bonus за уровень)
    let sp = &r["spells"][0];
    assert_eq!(sp["level"], 6, "{r}");
    // очки
    let p = &r["points"];
    assert_eq!(p["attributes"], 4, "{r}");
    assert_eq!(p["skills"], 7, "{r}");
    assert_eq!(p["ledger_spent"], 24, "{r}");
    assert_eq!(p["matches_ledger"], false, "{r}");
    // пререквизит техники (unarmed_a >= 10) не выполнен
    assert!(
        r["prereq_violations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|x| x["entry"] == "technique:strike_a"),
        "{r}"
    );

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
