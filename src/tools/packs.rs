//! Список паков.
use crate::db::Db;
use crate::error::ToolResult;
use serde_json::Value as J;

pub async fn list_packs(db: &Db) -> ToolResult<J> {
    db.one(
        "SELECT id, name, abbrev, kind, priority, owner, spell_system, notes FROM pack ORDER BY priority DESC, id;",
        vec![],
    )
    .await
}
