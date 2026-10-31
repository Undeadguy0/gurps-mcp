//! Персонаж с рёбрами (навыки, техники, заклинания, черты, предметы).
use super::{check_key, check_lang, default_lang};
use crate::db::Db;
use crate::error::{ToolError, ToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value as J;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCharacterParams {
    /// id персонажа без префикса таблицы, например `test_hero`.
    pub id: String,
    /// Язык названий в рёбрах (с откатом на en). По умолчанию ru.
    #[serde(default = "default_lang")]
    pub lang: String,
}

pub async fn get_character(db: &Db, p: GetCharacterParams) -> ToolResult<J> {
    check_key("id", &p.id)?;
    check_lang(&p.lang)?;
    let sql = "LET $c = type::record('character', $id);
        RETURN IF !record::exists($c) { NONE } ELSE { {
            character: $c.*,
            skills: (SELECT id, out AS target, fn::tr(out, $lang, 'name') AS name, points, spec, tl FROM has_skill WHERE in = $c),
            techniques: (SELECT id, out AS target, fn::tr(out, $lang, 'name') AS name, points FROM has_technique WHERE in = $c),
            spells: (SELECT id, out AS target, fn::tr(out, $lang, 'name') AS name, points FROM has_spell WHERE in = $c),
            traits: (SELECT id, out AS target, fn::tr(out, $lang, 'name') AS name, level, modifiers, note FROM has_trait WHERE in = $c),
            items: (SELECT id, out AS target, fn::tr(out, $lang, 'name') AS name, qty, equipped, location, contained_in FROM owns_item WHERE in = $c)
        } };";
    let v = db
        .one(sql, vec![("id", p.id.clone()), ("lang", p.lang)])
        .await?;
    if v.is_null() {
        return Err(ToolError::NotFound(format!("character:{}", p.id)));
    }
    Ok(v)
}
