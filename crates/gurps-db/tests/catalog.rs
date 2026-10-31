//! Каталог и вход расчёта на временной БД (surreal memory + surrealkit sync/seed).
#![cfg(feature = "testkit")]
use gurps_db::{calc_input, catalog, testkit};

#[tokio::test]
async fn catalog_loads_from_seed() {
    let Some((_srv, db)) = testkit::start().await else {
        return;
    };
    let packs = catalog::packs(&db, false).await.unwrap();
    assert!(packs.iter().any(|p| p.code == "basic"), "{packs:?}");
    assert!(packs.windows(2).all(|w| w[0].priority <= w[1].priority));

    let cat = catalog::Catalog::load(&db).await.unwrap();
    let skills = cat.table("skill");
    assert!(!skills.is_empty());
    for s in skills {
        assert!(s.ref_code("attribute").is_some(), "{}: attribute", s.id);
        assert!(s.ref_code("difficulty").is_some(), "{}: difficulty", s.id);
    }
    let first = &skills[0];
    assert_eq!(
        cat.effective("skill", &first.id.sid)
            .map(|e| e.id.sid.as_str()),
        Some(first.id.sid.as_str())
    );

    let attrs = catalog::refs(&db, "ref_attribute").await.unwrap();
    assert!(attrs.iter().any(|a| a.code == "DX"));
    assert!(catalog::refs(&db, "skill").await.is_err());
    assert!(catalog::fetch(&db, "character", "true").await.is_err());
}

#[tokio::test]
async fn calc_input_missing_character() {
    let Some((_srv, db)) = testkit::start().await else {
        return;
    };
    let e = calc_input::load(&db, "no_such_character")
        .await
        .unwrap_err();
    assert!(matches!(e, gurps_db::DbError::NotFound(_)), "{e}");
}
