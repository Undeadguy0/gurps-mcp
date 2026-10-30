//! Каталог: плагины образца -> БД -> плагины; повторный цикл стабилен.
mod common;
use common::*;
use sheet_bridge::catalog::{export, import};
use sheet_bridge::report::Dir;

/// Поля, которые схема заведомо не хранит (см. отчёт и README).
const LOSSY: &[&str] = &[
    "features",
    "prerequisites",
    "min_levels",
    "defaults",
    "skill_damage_bonus",
    "skill_modifier",
    "technique",
    "version",
];

#[tokio::test]
async fn catalog_roundtrip_sample() {
    let Some(sd) = sheet_dir() else {
        eprintln!("нет gurps-sheet, пропуск");
        return;
    };
    let (_s, db) = start().await;
    let r = import::run(&db, &[sd.join("plugins")], &catalog_opts(), false).await.unwrap();
    assert!(r.has(Dir::Import, "features.spell_bonus: нет аналога в схеме, отброшено"), "{r}");
    let out = tmp("cat1");
    export::run(&db, &["basic_set_sample".into()], &out).await.unwrap();
    let src = sd.join("plugins/basic-set-sample/data");
    for f in std::fs::read_dir(&src).unwrap().flatten() {
        let name = f.file_name();
        let mut raw = read_toml(&f.path());
        // описание оружия, привязанного к снаряжению, схема не хранит (отчёт: weapon.description)
        for w in raw.get_mut("weapons").and_then(|w| w.as_array_mut()).into_iter().flatten() {
            if w.get("equipment").is_some() {
                w.as_object_mut().unwrap().remove("description");
            }
        }
        // нераспознаваемые страницы ("example") отбрасываются с отчётом
        for list in raw.as_object_mut().unwrap().values_mut().filter_map(|v| v.as_array_mut()) {
            for e in list.iter_mut().filter_map(|e| e.as_object_mut()) {
                if e.get("page")
                    .and_then(|p| p.as_str())
                    .is_some_and(|p| !p.starts_with(|c: char| c.is_ascii_uppercase()))
                {
                    e.remove("page");
                }
            }
        }
        let a = norm(&raw, LOSSY);
        let b = norm(&read_toml(&out.join("basic-set-sample/data").join(&name)), LOSSY);
        // поля, добавленные экспортом по умолчанию, допустимы: сравниваем исходные ключи
        if let Some(d) = diff_sub(&a, &b, &format!("{name:?}")) {
            panic!("расхождение: {d}");
        }
    }
    // описания из .ftl пережили цикл
    // переводы lang-ru привязаны к записям и экспортируются в .ftl пака
    let ftl = std::fs::read_to_string(out.join("basic-set-sample/locales/ru/basic-set-sample.ftl")).unwrap();
    let got = sheet_bridge::sheet::ftl::parse_ftl(&ftl, "out").unwrap();
    let mut want = std::collections::BTreeMap::new();
    for f in std::fs::read_dir(sd.join("plugins/lang-ru/locales/ru")).unwrap().flatten() {
        want.extend(sheet_bridge::sheet::ftl::parse_ftl(&std::fs::read_to_string(f.path()).unwrap(), "src").unwrap());
    }
    // переводы оружия, связанного со снаряжением, теряются (в схеме нет записи оружия)
    let lost = ["weapon-broadsword", "weapon-small-knife", "weapon-large-knife"];
    for (k, v) in &want {
        if lost.contains(&k.as_str()) {
            continue;
        }
        let g = got.get(k).unwrap_or_else(|| panic!("нет перевода {k}"));
        assert_eq!(g.name, v.name, "{k}");
        assert_eq!(
            g.description.as_deref().map(str::trim),
            v.description.as_deref().map(str::trim),
            "{k}"
        );
    }

    // второй цикл: экспорт -> импорт -> экспорт без изменений
    let (_s2, db2) = start().await;
    import::run(&db2, &[out.join("basic-set-sample")], &catalog_opts(), false)
        .await
        .unwrap();
    let out2 = tmp("cat2");
    export::run(&db2, &["basic_set_sample".into()], &out2).await.unwrap();
    for f in std::fs::read_dir(out.join("basic-set-sample/data")).unwrap().flatten() {
        let a = read_toml(&f.path());
        let b = read_toml(&out2.join("basic-set-sample/data").join(f.file_name()));
        assert_eq!(a, b, "{:?} нестабилен", f.file_name());
    }
}

/// Каждое значение исходника присутствует в результате (результат может содержать умолчания).
fn diff_sub(a: &serde_json::Value, b: &serde_json::Value, path: &str) -> Option<String> {
    use serde_json::Value as J;
    match (a, b) {
        (J::Object(x), J::Object(y)) => x.iter().find_map(|(k, l)| match y.get(k) {
            Some(r) => diff_sub(l, r, &format!("{path}.{k}")),
            None => Some(format!("{path}.{k}: нет в результате ({l})")),
        }),
        (J::Array(x), J::Array(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .enumerate()
            .find_map(|(i, (l, r))| diff_sub(l, r, &format!("{path}[{i}]"))),
        _ => diff(a, b, path),
    }
}

#[tokio::test]
async fn catalog_dry_run_writes_nothing() {
    let Some(sd) = sheet_dir() else { return };
    let (_s, db) = start().await;
    import::run(&db, &[sd.join("plugins")], &catalog_opts(), true).await.unwrap();
    let n = db.one("RETURN count(SELECT id FROM skill);").await.unwrap();
    assert_eq!(n, serde_json::json!(0));
    assert_eq!(
        db.one("RETURN count(SELECT id FROM pack:basic_set_sample);").await.unwrap(),
        serde_json::json!(0)
    );
}
