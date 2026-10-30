//! Расчёт персонажа и проверка пререквизитов (модуль `calc`).
use super::{CatalogTable, check_key};
use crate::calc::{self, Missing, loader, prereq};
use crate::db::Db;
use crate::error::{ToolError, ToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value as J;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComputeCharacterParams {
    /// id персонажа без префикса таблицы, например `test_hero`.
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckPrereqsParams {
    /// id персонажа без префикса таблицы.
    pub character: String,
    /// Таблица каталога записи.
    pub table: CatalogTable,
    /// sid записи (действующая версия по приоритету паков).
    pub sid: String,
}

pub async fn compute_character(db: &Db, p: ComputeCharacterParams) -> ToolResult<J> {
    check_key("id", &p.id)?;
    let inp = loader::load(db, &p.id).await?;
    serde_json::to_value(calc::compute(&inp)).map_err(|e| ToolError::Db(e.to_string()))
}

pub async fn check_prereqs(db: &Db, p: CheckPrereqsParams) -> ToolResult<J> {
    check_key("character", &p.character)?;
    check_key("sid", &p.sid)?;
    let entry = db
        .one(
            "LET $e = fn::effective($tb, $sid); RETURN IF $e = NONE { NONE } ELSE { { id: $e.id, prereqs: $e.prereqs } };",
            vec![("tb", p.table.as_str().to_string()), ("sid", p.sid.clone())],
        )
        .await?;
    if entry.is_null() {
        return Err(ToolError::NotFound(format!(
            "{}:{}",
            p.table.as_str(),
            p.sid
        )));
    }
    let inp = loader::load(db, &p.character).await?;
    let mut miss = Missing::default();
    let fx = calc::features::Effects::collect(&inp);
    let attrs = calc::attributes::compute(&inp, &fx, &mut miss);
    let sk = calc::skills::Skills::compute(&inp, &attrs, &fx, &mut miss);
    let ctx = prereq::Ctx::new(&inp, &attrs, &sk, &mut miss);
    let unmet = entry
        .get("prereqs")
        .filter(|v| !v.is_null())
        .map(|n| prereq::eval(n, &ctx))
        .unwrap_or_default();
    Ok(serde_json::json!({
        "entry": entry["id"],
        "prereqs": entry["prereqs"],
        "ok": unmet.is_empty(),
        "unmet": unmet,
        "missing_constants": miss,
    }))
}
