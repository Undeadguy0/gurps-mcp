//! Инструменты напрямую против surreal memory со схемой и сидами.
mod common;
use gurps_mcp::error::ToolError;
use gurps_mcp::tools::{
    CatalogTable, character::GetCharacterParams, entry::GetEntryParams, search::SearchParams,
};

#[tokio::test]
async fn tools_against_seeded_db() {
    let Some((_s, db)) = common::start().await else {
        return;
    };

    // list_packs: по убыванию приоритета
    let packs = gurps_mcp::tools::packs::list_packs(&db).await.unwrap();
    let ids: Vec<String> = packs
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["id"].to_string())
        .collect();
    assert!(ids.len() >= 3, "{packs}");
    assert!(ids[0].contains("custom_test_hero"), "{ids:?}");

    // search: en, фильтр по таблице
    let r = gurps_mcp::tools::search::search(
        &db,
        SearchParams {
            query: "sword".into(),
            table: Some(CatalogTable::Skill),
            lang: "en".into(),
            limit: Some(10),
        },
    )
    .await
    .unwrap();
    let res = r["results"].as_array().unwrap();
    assert!(!res.is_empty(), "{r}");
    assert!(res.iter().all(|x| x["table"] == "skill"), "{r}");
    // search: ru со стеммингом
    let r = gurps_mcp::tools::search::search(
        &db,
        SearchParams {
            query: "специализац".into(),
            table: None,
            lang: "ru".into(),
            limit: None,
        },
    )
    .await
    .unwrap();
    assert!(!r["results"].as_array().unwrap().is_empty(), "{r}");
    // ru: точная словоформа находится полнотекстом
    let r = gurps_mcp::tools::search::search(
        &db,
        SearchParams {
            query: "заглушка".into(),
            table: Some(CatalogTable::Trait),
            lang: "ru".into(),
            limit: None,
        },
    )
    .await
    .unwrap();
    assert!(!r["results"].as_array().unwrap().is_empty(), "{r}");
    // пустой запрос — ошибка параметров
    let e = gurps_mcp::tools::search::search(
        &db,
        SearchParams {
            query: " ".into(),
            table: None,
            lang: "ru".into(),
            limit: None,
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(e, ToolError::InvalidParams(_)));

    // get_entry: override по приоритету -> homebrew
    let e = gurps_mcp::tools::entry::get_entry(
        &db,
        GetEntryParams {
            table: CatalogTable::Skill,
            sid: "sword_a".into(),
            pack: None,
        },
    )
    .await
    .unwrap();
    assert!(e["record"]["id"].to_string().contains("homebrew"), "{e}");
    assert_eq!(e["alternatives"].as_array().unwrap().len(), 2, "{e}");
    assert!(!e["translations"].as_array().unwrap().is_empty(), "{e}");
    // get_entry: явный пак
    let e = gurps_mcp::tools::entry::get_entry(
        &db,
        GetEntryParams {
            table: CatalogTable::Skill,
            sid: "sword_a".into(),
            pack: Some("basic".into()),
        },
    )
    .await
    .unwrap();
    assert!(e["record"]["id"].to_string().contains("basic"), "{e}");
    assert!(
        e["translations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|t| t["field"] == "description"),
        "{e}"
    );
    // get_entry: нет записи
    let e = gurps_mcp::tools::entry::get_entry(
        &db,
        GetEntryParams {
            table: CatalogTable::Spell,
            sid: "nope".into(),
            pack: None,
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(e, ToolError::NotFound(_)), "{e}");

    // get_character с рёбрами
    let c = gurps_mcp::tools::character::get_character(
        &db,
        GetCharacterParams {
            id: "test_hero".into(),
            lang: "en".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(c["character"]["name"], "Тестовый герой", "{c}");
    assert_eq!(c["skills"].as_array().unwrap().len(), 3, "{c}");
    assert_eq!(c["traits"].as_array().unwrap().len(), 3, "{c}");
    assert_eq!(c["items"].as_array().unwrap().len(), 3, "{c}");
    assert!(
        c["skills"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["name"] == "Sword A"),
        "{c}"
    );
    let e = gurps_mcp::tools::character::get_character(
        &db,
        GetCharacterParams {
            id: "nobody".into(),
            lang: "en".into(),
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(e, ToolError::NotFound(_)));
}
