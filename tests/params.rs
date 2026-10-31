//! Проверка параметров без БД.
use gurps_mcp::tools::{CatalogTable, search::SearchParams};

#[test]
fn search_params_defaults() {
    let p: SearchParams =
        serde_json::from_value(serde_json::json!({ "query": "x", "table": "trait" })).unwrap();
    assert_eq!(p.lang, "ru");
    assert_eq!(p.table, Some(CatalogTable::Trait));
    assert!(p.limit.is_none());
}

#[test]
fn unknown_table_rejected() {
    let r: Result<SearchParams, _> =
        serde_json::from_value(serde_json::json!({ "query": "x", "table": "pack" }));
    assert!(r.is_err());
}

#[test]
fn schema_has_enum() {
    let s = serde_json::to_value(schemars::schema_for!(SearchParams))
        .unwrap()
        .to_string();
    assert!(s.contains("technique"), "{s}");
}
