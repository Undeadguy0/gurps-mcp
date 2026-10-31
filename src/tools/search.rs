//! Полнотекстовый поиск по translation.text_<lang> (отдельный индекс и анализатор на язык).
use super::{CatalogTable, FT_LANGS, check_lang, default_lang};
use crate::db::Db;
use crate::error::{ToolError, ToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value as J;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Текст запроса (слова; стемминг по языку).
    pub query: String,
    /// Ограничить таблицей каталога.
    #[serde(default)]
    pub table: Option<CatalogTable>,
    /// Язык переводов, по которым искать: ru или en. По умолчанию ru.
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
    if !FT_LANGS.contains(&p.lang.as_str()) {
        return Err(ToolError::InvalidParams(format!(
            "lang: полнотекстовый индекс есть только для {FT_LANGS:?}, получено {:?}",
            p.lang
        )));
    }
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
    // Поле и индекс — из перечисления FT_LANGS; LIMIT — проверенное число; таблицы — из перечисления.
    let field = format!("text_{}", p.lang);
    let sql = format!(
        "SELECT record, record::tb(record) AS table, record.pack AS pack, record.sid AS sid,
                field, text, search::score(1) AS score
         FROM translation
         WHERE {field} @1@ $q AND record::tb(record) IN [{tb_list}]
         ORDER BY score DESC LIMIT {limit};"
    );
    let rows = db.one(&sql, vec![("q", q.to_string())]).await?;
    Ok(serde_json::json!({ "query": q, "lang": p.lang, "results": rows }))
}
