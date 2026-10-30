//! Полнотекстовый поиск по translation.text.
use super::{CatalogTable, check_lang, default_lang};
use crate::db::Db;
use crate::error::{ToolError, ToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value as J;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Текст запроса (слова; стемминг en/ru).
    pub query: String,
    /// Ограничить таблицей каталога.
    #[serde(default)]
    pub table: Option<CatalogTable>,
    /// Язык переводов, по которым искать (en, ru, …). По умолчанию ru.
    #[serde(default = "default_lang")]
    pub lang: String,
    /// Сколько результатов вернуть (1..=100, по умолчанию 20).
    #[serde(default)]
    pub limit: Option<u32>,
}

pub async fn search(db: &Db, p: SearchParams) -> ToolResult<J> {
    let q = p.query.trim();
    if q.is_empty() {
        return Err(ToolError::InvalidParams("query: пустой запрос".into()));
    }
    check_lang(&p.lang)?;
    let limit = p.limit.unwrap_or(20).clamp(1, 100);
    let tables: Vec<&str> = match p.table {
        Some(t) => vec![t.as_str()],
        None => vec!["skill", "trait", "item", "spell", "technique"],
    };
    let tb_list = tables
        .iter()
        .map(|t| format!("'{t}'"))
        .collect::<Vec<_>>()
        .join(", ");
    // LIMIT — литерал (проверенное число), таблицы — из перечисления.
    let sql = format!(
        "SELECT record, record::tb(record) AS table, record.pack AS pack, record.sid AS sid,
                field, text, search::score(1) AS score
         FROM translation
         WHERE lang = $lang AND text @1@ $q AND record::tb(record) IN [{tb_list}]
         ORDER BY score DESC LIMIT {limit};"
    );
    let vars = || vec![("lang", p.lang.clone()), ("q", q.to_string())];
    let mut rows = db.one(&sql, vars()).await?;
    let mut mode = "fulltext";
    // Индексы en/ru висят на одном поле: для словоформ другого языка FT может не сработать —
    // тогда откат на подстроку без учёта регистра.
    if rows.as_array().is_some_and(|a| a.is_empty()) {
        mode = "substring";
        let sql = format!(
            "SELECT record, record::tb(record) AS table, record.pack AS pack, record.sid AS sid,
                    field, text, 0 AS score
             FROM translation
             WHERE lang = $lang AND string::lowercase(text) CONTAINS string::lowercase($q)
               AND record::tb(record) IN [{tb_list}]
             LIMIT {limit};"
        );
        rows = db.one(&sql, vars()).await?;
    }
    Ok(serde_json::json!({ "query": q, "lang": p.lang, "mode": mode, "results": rows }))
}
