//! Каталожная запись: действующая по приоритету пака или из явного пака.
use super::{CatalogTable, check_key};
use crate::db::Db;
use crate::error::{ToolError, ToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value as J;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetEntryParams {
    /// Таблица каталога.
    pub table: CatalogTable,
    /// sid записи (вторая часть id ['pack','sid']).
    pub sid: String,
    /// Пак. Если не задан — берётся пак с наибольшим priority (fn::effective).
    #[serde(default)]
    pub pack: Option<String>,
}

pub async fn get_entry(db: &Db, p: GetEntryParams) -> ToolResult<J> {
    check_key("sid", &p.sid)?;
    let tb = p.table.as_str().to_string();
    let mut vars = vec![("tb", tb), ("sid", p.sid.clone())];
    let pick = match &p.pack {
        Some(pack) => {
            check_key("pack", pack)?;
            vars.push(("pack", pack.clone()));
            "type::record($tb, [$pack, $sid]).*"
        }
        None => "fn::effective($tb, $sid)",
    };
    let by_prio = p.pack.is_none();
    let sql = format!(
        "LET $e = {pick};
         RETURN IF $e = NONE {{ NONE }} ELSE {{ {{
             record: $e,
             resolved_by_priority: {by_prio},
             alternatives: (SELECT VALUE id FROM type::table($tb) WHERE sid = $sid),
             translations: (SELECT lang, field, text FROM translation WHERE record = $e.id ORDER BY lang, field)
         }} }};"
    );
    let v = db.one(&sql, vars).await?;
    if v.is_null() {
        let at = p.pack.map(|x| format!(" в паке {x}")).unwrap_or_default();
        return Err(ToolError::NotFound(format!(
            "{}:{}{at}",
            p.table.as_str(),
            p.sid
        )));
    }
    Ok(v)
}
