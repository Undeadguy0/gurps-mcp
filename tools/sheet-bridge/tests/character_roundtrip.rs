//! Персонажи: golden-файлы gurps-sheet -> БД -> файл; повторный цикл стабилен.
mod common;
use common::*;
use serde_json::Value as J;
use sheet_bridge::catalog::import as cimport;
use sheet_bridge::character::{export, import};
use sheet_bridge::db::Db;
use std::path::{Path, PathBuf};

/// Поля, которые схема не хранит или которые экспорт выводит иначе (отчёт + README).
const LOSSY: &[&str] = &[
    "features",
    "prerequisites",
    "min_levels",
    "weapons",
    "description",
    "source",
    "defaults",
    "skill_damage_bonus",
    "skill_modifier",
    "technique",
];

fn goldens(sd: &Path) -> Vec<PathBuf> {
    let mut v = vec![sd.join("examples/hero.json")];
    let mut t: Vec<_> = std::fs::read_dir(sd.join("examples/test-chars"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    t.sort();
    v.extend(t);
    v
}

/// Умолчания, добавляемые экспортом (attribute заклинания, id снаряжения без id).
fn fill(mut j: J) -> J {
    let c = &mut j["character"];
    for s in c["spells"].as_array_mut().into_iter().flatten() {
        if s.get("attribute").is_none() {
            s["attribute"] = "IQ".into();
        }
    }
    for t in c["traits"].as_array_mut().into_iter().flatten() {
        if t["cost"].is_null() {
            t["cost"] = 0.into(); // в схеме cost обязателен: null -> 0
        }
    }
    for e in c["equipment"].as_array_mut().into_iter().flatten() {
        if e.get("id").is_none() {
            let n = e["name"].as_str().unwrap_or("").to_lowercase().replace(' ', "-");
            e["id"] = n.into();
        }
    }
    j
}

async fn cycle(db: &Db, file: &Path, out: &Path) -> J {
    let id = import::char_id(file, None).unwrap();
    import::run(db, file, &char_opts(None), false)
        .await
        .unwrap_or_else(|e| panic!("{}: {e:#}", file.display()));
    let dst = out.join(format!("{id}.json"));
    export::run(db, &id, &dst).await.unwrap();
    serde_json::from_str(&std::fs::read_to_string(&dst).unwrap()).unwrap()
}

#[tokio::test]
async fn golden_characters_roundtrip() {
    let Some(sd) = sheet_dir() else {
        eprintln!("нет gurps-sheet, пропуск");
        return;
    };
    let (_s, db) = start().await;
    cimport::run(&db, &[sd.join("plugins")], &catalog_opts(), false).await.unwrap();
    let out1 = tmp("ch1");
    let out2 = tmp("ch2");
    let files = goldens(&sd);
    assert!(files.len() >= 4);
    for f in &files {
        let orig: J = fill(serde_json::from_str(&std::fs::read_to_string(f).unwrap()).unwrap());
        let a = cycle(&db, f, &out1).await;
        // страница "example" и пр. нераспознаваемые — отбрасываются
        let mut o = norm(&orig, LOSSY);
        strip_bad_pages(&mut o);
        let na = norm(&a, LOSSY);
        if let Some(d) = diff(&o, &na, &f.display().to_string()) {
            panic!("цикл 1: {d}");
        }
        // второй цикл байт-в-байт
        let id = import::char_id(f, None).unwrap();
        let p1 = out1.join(format!("{id}.json"));
        let b = cycle(&db, &p1, &out2).await;
        assert_eq!(a, b, "{} нестабилен", f.display());
    }
}

fn strip_bad_pages(v: &mut J) {
    match v {
        J::Object(o) => {
            if o.get("page")
                .and_then(|p| p.as_str())
                .is_some_and(|p| !p.starts_with(|c: char| c.is_ascii_uppercase()))
            {
                o.remove("page");
            }
            o.values_mut().for_each(strip_bad_pages);
        }
        J::Array(a) => a.iter_mut().for_each(strip_bad_pages),
        _ => {}
    }
}

#[tokio::test]
async fn ledger_and_custom_roundtrip() {
    let Some(sd) = sheet_dir() else { return };
    let (_s, db) = start().await;
    cimport::run(&db, &[sd.join("plugins")], &catalog_opts(), false).await.unwrap();
    let mut j: J = serde_json::from_str(&std::fs::read_to_string(sd.join("examples/test-chars/warrior-skills.json")).unwrap()).unwrap();
    let skill = j["character"]["skills"][0]["id"].clone();
    j["character"]["ledger"] = serde_json::json!([
        { "id": 1, "date": "2026-01-01", "amount": 150, "kind": "starting", "note": "старт" },
        { "id": 2, "date": "2026-02-01", "amount": 5, "kind": "session-award", "note": "сессия 1" },
        { "id": 3, "date": "2026-02-02", "amount": -2, "kind": "impulse-buy", "note": "трата", "skill": skill }
    ]);
    // своё определение, которого нет в каталоге
    j["character"]["traits"][0]["id"] = "homebrew-luck".into();
    j["character"]["traits"][0]["source"] = J::Null;
    let dir = tmp("ledger");
    let src = dir.join("ledger-hero.json");
    std::fs::write(&src, serde_json::to_string_pretty(&j).unwrap()).unwrap();
    let a = cycle(&db, &src, &dir.join("o")).await;
    let c = &a["character"];
    assert_eq!(c["ledger"].as_array().unwrap().len(), 3);
    assert_eq!(c["ledger"][2]["skill"], skill);
    assert_eq!(c["ledger"][1]["kind"], "session-award");
    assert_eq!(c["points_budget"], 153);
    assert_eq!(c["traits"][0]["id"], "homebrew-luck");
    let n = db
        .one("RETURN count(SELECT id FROM trait WHERE pack = pack:custom_ledger_hero);")
        .await
        .unwrap();
    assert_eq!(n, serde_json::json!(1));
    let k = db
        .one("RETURN (SELECT VALUE kind FROM ONLY pack:custom_ledger_hero);")
        .await
        .unwrap();
    assert_eq!(k, serde_json::json!("custom"));
    // повторный импорт не плодит записи
    import::run(&db, &src, &char_opts(None), false).await.unwrap();
    let n = db
        .one("RETURN count(SELECT id FROM point_ledger WHERE character = character:ledger_hero);")
        .await
        .unwrap();
    assert_eq!(n, serde_json::json!(3));
}

#[tokio::test]
async fn character_dry_run_writes_nothing() {
    let Some(sd) = sheet_dir() else { return };
    let (_s, db) = start().await;
    cimport::run(&db, &[sd.join("plugins")], &catalog_opts(), false).await.unwrap();
    import::run(&db, &sd.join("examples/test-chars/warrior.json"), &char_opts(None), true)
        .await
        .unwrap();
    assert_eq!(
        db.one("RETURN count(SELECT id FROM character);").await.unwrap(),
        serde_json::json!(0)
    );
    assert_eq!(
        db.one("RETURN count(SELECT id FROM has_trait);").await.unwrap(),
        serde_json::json!(0)
    );
}
