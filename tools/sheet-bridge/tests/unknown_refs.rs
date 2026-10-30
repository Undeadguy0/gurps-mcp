//! Неизвестные коды справочников: импорт падает с понятной ошибкой и ничего не создаёт.
mod common;
use common::*;
use sheet_bridge::catalog::import;
use std::path::PathBuf;

fn plugin(name: &str, kind: &str, body: &str) -> PathBuf {
    let d = tmp(name);
    std::fs::write(
        d.join("plugin.toml"),
        format!("id = \"{name}\"\nname = \"T\"\nversion = \"0.1.0\"\ndepends = []\n"),
    )
    .unwrap();
    std::fs::create_dir_all(d.join("data")).unwrap();
    std::fs::write(d.join("data").join(format!("{kind}.toml")), body).unwrap();
    d
}

async fn expect_fail(p: PathBuf, needle: &str) {
    let (_s, db) = start().await;
    let err = import::run(&db, &[p], &catalog_opts(), false)
        .await
        .expect_err("импорт должен упасть");
    let msg = format!("{err:#}");
    assert!(msg.contains(needle), "в ошибке нет {needle}: {msg}");
    for t in ["skill", "spell", "pack", "ref_attribute", "ref_college", "ref_difficulty"] {
        let n = db.one(&format!("RETURN count(SELECT id FROM {t} WHERE string::contains(record::id(id), 'bogus') OR string::contains(<string>id, 'necromancy') OR string::contains(<string>id, 'BasicLift'));")).await.unwrap();
        assert_eq!(n, serde_json::json!(0), "{t}: создано лишнее");
    }
    assert_eq!(db.one("RETURN count(SELECT id FROM pack WHERE kind != 'official' OR id = pack:bogus_attr OR id = pack:bogus_college OR id = pack:bogus_diff);").await.unwrap(), serde_json::json!(1), "лишний пак");
}

#[tokio::test]
async fn unknown_attribute_fails() {
    let p = plugin(
        "bogus-attr",
        "skills",
        "[[skills]]\nid = \"lifting\"\nname = \"Lifting\"\nattribute = \"BasicLift\"\ndifficulty = \"A\"\n",
    );
    expect_fail(p, "BasicLift").await;
}

#[tokio::test]
async fn unknown_college_fails() {
    let p = plugin(
        "bogus-college",
        "spells",
        "[[spells]]\nid = \"zombie\"\nname = \"Zombie\"\ncollege = [\"Necromancy\"]\ndifficulty = \"H\"\n",
    );
    expect_fail(p, "necromancy").await;
}

#[tokio::test]
async fn unknown_difficulty_fails() {
    let p = plugin(
        "bogus-diff",
        "skills",
        "[[skills]]\nid = \"x\"\nname = \"X\"\nattribute = \"DX\"\ndifficulty = \"Q\"\n",
    );
    expect_fail(p, "Q").await;
}

#[tokio::test]
async fn character_unknown_ref_fails() {
    let Some(sd) = sheet_dir() else { return };
    let (_s, db) = start().await;
    let mut j: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(sd.join("examples/test-chars/warrior-skills.json")).unwrap()).unwrap();
    j["character"]["skills"][0]["id"] = "bogus-lifting".into();
    j["character"]["skills"][0]["attribute"] = "BasicLift".into();
    let d = tmp("bad-char");
    let f = d.join("bad.json");
    std::fs::write(&f, j.to_string()).unwrap();
    let err = sheet_bridge::character::import::run(&db, &f, &char_opts(None), false)
        .await
        .expect_err("должно упасть");
    assert!(format!("{err:#}").contains("BasicLift"), "{err:#}");
    assert_eq!(
        db.one("RETURN count(SELECT id FROM character);").await.unwrap(),
        serde_json::json!(0)
    );
    assert_eq!(
        db.one("RETURN count(SELECT id FROM pack WHERE kind = 'custom');").await.unwrap(),
        serde_json::json!(0)
    );
}
