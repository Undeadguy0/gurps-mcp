//! Каталог (только чтение): паки, записи `(pack, sid)`, переводы, справочники `ref_*`.
//!
//! Каталог правится только сидами/MCP; редактор читает его как есть. Записи отдаются в
//! «собранной» форме: поля записи, как их отдаёт сервер, плюс служебные `_tr`
//! (lang -> field -> text), `_attr_defaults` и `_defaults_to`.
use crate::db::Db;
use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};
use serde_json::Value as J;
use std::collections::BTreeMap;

/// Таблицы каталога (порядок зависимостей: на что ссылаются — раньше).
pub const TABLES: [&str; 7] = [
    "skill",
    "technique",
    "modifier",
    "trait",
    "attack_mode",
    "item",
    "spell",
];

/// Переводы записи: lang -> field (`name`, `description`, `notes`, `spec.<sid>`) -> text.
pub type Translations = BTreeMap<String, BTreeMap<String, String>>;

/// Ключ записи каталога: `table:[pack, sid]`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CatalogId {
    pub pack: String,
    pub sid: String,
}

impl CatalogId {
    pub fn new(pack: impl Into<String>, sid: impl Into<String>) -> Self {
        Self {
            pack: pack.into(),
            sid: sid.into(),
        }
    }
}

impl std::fmt::Display for CatalogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.pack, self.sid)
    }
}

/// Строка ссылки `table:key` (как её отдаёт JSON SurrealDB) -> (table, key-часть).
fn split_record(s: &str) -> Option<(&str, &str)> {
    let (t, k) = s.split_once(':')?;
    (!t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')).then_some((t, k))
}

fn unquote(s: &str) -> &str {
    let s = s.trim();
    let s = s
        .strip_prefix('⟨')
        .and_then(|x| x.strip_suffix('⟩'))
        .unwrap_or(s);
    s.trim_matches(|c| c == '\'' || c == '"' || c == '`')
}

/// `skill:['basic', 'sword_a']` -> ("skill", basic/sword_a).
pub fn parse_catalog_record(s: &str) -> Option<(&str, CatalogId)> {
    let (t, k) = split_record(s)?;
    let inner = k.trim().strip_prefix('[')?.strip_suffix(']')?;
    let (p, sid) = inner.split_once(',')?;
    Some((t, CatalogId::new(unquote(p), unquote(sid))))
}

/// `ref_attribute:DX` -> "DX", `pack:basic` -> "basic" (код справочной записи).
pub fn record_code(s: &str) -> Option<&str> {
    let (_, k) = split_record(s)?;
    (!k.starts_with('[')).then(|| unquote(k))
}

fn check_ident(what: &str, s: &str) -> DbResult<()> {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        Ok(())
    } else {
        Err(DbError::Invalid(format!("{what}: {s:?}")))
    }
}

/// Пак-источник данных.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pack {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub abbrev: Option<String>,
    pub kind: String,
    #[serde(default)]
    pub priority: i64,
}

/// Паки по возрастанию приоритета (последний перекрывает предыдущие). `custom` — по флагу.
pub async fn packs(db: &Db, include_custom: bool) -> DbResult<Vec<Pack>> {
    let filter = if include_custom {
        "true"
    } else {
        "kind != 'custom'"
    };
    let v = db
        .one(
            &format!(
                "SELECT record::id(id) AS code, name, abbrev, kind, priority FROM pack \
                 WHERE {filter} ORDER BY priority, code;"
            ),
            vec![],
        )
        .await?;
    serde_json::from_value(v).map_err(|e| DbError::Decode(format!("pack: {e}")))
}

/// Запрос «собранных» записей таблицы каталога с фильтром (SurrealQL-условие).
pub fn assembled_query(table: &str, filter: &str) -> String {
    format!(
        "SELECT *, \
         (SELECT lang, field, text FROM translation WHERE record = $parent.id) AS _trl, \
         (SELECT attribute, mod FROM attr_default WHERE skill = $parent.id ORDER BY attribute) AS _attr_defaults, \
         (SELECT out, mod, any_spec, spec, from_spec, cap FROM defaults_to WHERE in = $parent.id ORDER BY out) AS _defaults_to \
         FROM {table} WHERE {filter} ORDER BY sid;"
    )
}

fn take_tr(list: Option<&J>) -> Translations {
    let mut m = Translations::new();
    for t in list.and_then(J::as_array).into_iter().flatten() {
        let s = |k: &str| t.get(k).and_then(J::as_str);
        if let (Some(l), Some(f), Some(x)) = (s("lang"), s("field"), s("text")) {
            m.entry(l.into()).or_default().insert(f.into(), x.into());
        }
    }
    m
}

/// `_trl` (список) -> `_tr` (lang -> field -> text).
pub fn fold_tr(mut j: J) -> J {
    if let Some(o) = j.as_object_mut() {
        let tr = take_tr(o.remove("_trl").as_ref());
        o.insert("_tr".into(), serde_json::to_value(tr).unwrap_or_default());
    }
    j
}

/// Собранные записи таблицы каталога. `filter` — доверенное условие (не ввод пользователя).
pub async fn fetch(db: &Db, table: &str, filter: &str) -> DbResult<Vec<J>> {
    if !TABLES.contains(&table) {
        return Err(DbError::Invalid(format!("таблица каталога: {table}")));
    }
    let v = db.one(&assembled_query(table, filter), vec![]).await?;
    Ok(v.as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(fold_tr)
        .collect())
}

/// Все собранные записи пака по таблицам каталога.
pub async fn fetch_pack(db: &Db, pack: &str) -> DbResult<BTreeMap<&'static str, Vec<J>>> {
    check_ident("pack", pack)?;
    let mut rows = BTreeMap::new();
    for t in TABLES {
        rows.insert(t, fetch(db, t, &format!("pack = pack:{pack}")).await?);
    }
    Ok(rows)
}

/// Одна запись каталога.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Entry {
    pub table: &'static str,
    pub id: CatalogId,
    pub tr: Translations,
    /// Собранный JSON записи (ссылки — строки `table:key`).
    pub data: J,
}

impl Entry {
    pub fn from_assembled(table: &'static str, data: J) -> DbResult<Self> {
        let id = data
            .get("id")
            .and_then(J::as_str)
            .and_then(parse_catalog_record)
            .map(|(_, id)| id)
            .ok_or_else(|| DbError::Decode(format!("{table}: нет id [pack, sid]")))?;
        let tr: Translations = data
            .get("_tr")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .map_err(|e| DbError::Decode(format!("{table} {id}: _tr: {e}")))?
            .unwrap_or_default();
        Ok(Self {
            table,
            id,
            tr,
            data,
        })
    }

    /// Текст поля перевода: `lang`, иначе `en`.
    pub fn text(&self, lang: &str, field: &str) -> Option<&str> {
        [lang, "en"]
            .iter()
            .find_map(|l| self.tr.get(*l).and_then(|m| m.get(field)))
            .map(String::as_str)
    }

    /// Код ссылки на справочник в поле `field` (`ref_attribute:DX` -> "DX").
    pub fn ref_code(&self, field: &str) -> Option<&str> {
        self.data
            .get(field)
            .and_then(J::as_str)
            .and_then(record_code)
    }
}

/// Весь каталог (не-custom паки) в памяти.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    /// По возрастанию приоритета.
    pub packs: Vec<Pack>,
    /// Таблица -> записи всех паков (порядок паков, внутри — по sid).
    pub entries: BTreeMap<&'static str, Vec<Entry>>,
    /// Сырые собранные строки по пакам (для адаптеров, которым нужен JSON целиком).
    pub rows: BTreeMap<String, BTreeMap<&'static str, Vec<J>>>,
}

impl Catalog {
    pub async fn load(db: &Db) -> DbResult<Self> {
        let packs = packs(db, false).await?;
        let mut entries: BTreeMap<&'static str, Vec<Entry>> = BTreeMap::new();
        let mut rows = BTreeMap::new();
        for p in &packs {
            let r = fetch_pack(db, &p.code).await?;
            for (t, js) in &r {
                for j in js {
                    entries
                        .entry(t)
                        .or_default()
                        .push(Entry::from_assembled(t, j.clone())?);
                }
            }
            rows.insert(p.code.clone(), r);
        }
        Ok(Self {
            packs,
            entries,
            rows,
        })
    }

    pub fn table(&self, table: &str) -> &[Entry] {
        self.entries.get(table).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn get(&self, table: &str, id: &CatalogId) -> Option<&Entry> {
        self.table(table).iter().find(|e| &e.id == id)
    }

    fn priority(&self, pack: &str) -> i64 {
        self.packs
            .iter()
            .find(|p| p.code == pack)
            .map_or(i64::MIN, |p| p.priority)
    }

    /// Действующая запись по sid — из пака с наибольшим priority (как `fn::effective`).
    pub fn effective(&self, table: &str, sid: &str) -> Option<&Entry> {
        self.table(table)
            .iter()
            .filter(|e| e.id.sid == sid)
            .max_by_key(|e| self.priority(&e.id.pack))
    }
}

/// Строка справочника `ref_*`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RefRow {
    pub code: String,
    pub tr: Translations,
    pub data: J,
}

/// Все строки справочника (`ref_attribute`, `ref_difficulty`, `ref_trait_category`, …).
pub async fn refs(db: &Db, table: &str) -> DbResult<Vec<RefRow>> {
    if !table.starts_with("ref_") {
        return Err(DbError::Invalid(format!("справочник: {table}")));
    }
    check_ident("справочник", table)?;
    let v = db
        .one(
            &format!(
                "SELECT *, record::id(id) AS _code, \
                 (SELECT lang, field, text FROM translation WHERE record = $parent.id) AS _trl \
                 FROM {table} ORDER BY _code;"
            ),
            vec![],
        )
        .await?;
    let mut out = vec![];
    for mut j in v.as_array().cloned().unwrap_or_default() {
        let o = j
            .as_object_mut()
            .ok_or_else(|| DbError::Decode(format!("{table}: не объект")))?;
        let code = match o.remove("_code") {
            Some(J::String(s)) => s,
            Some(x) => x.to_string(),
            None => return Err(DbError::Decode(format!("{table}: нет id"))),
        };
        let tr = take_tr(o.remove("_trl").as_ref());
        out.push(RefRow { code, tr, data: j });
    }
    Ok(out)
}

/// Названия колледжей магии (en).
pub async fn college_names(db: &Db) -> DbResult<BTreeMap<String, String>> {
    Ok(refs(db, "ref_college")
        .await?
        .into_iter()
        .filter_map(|r| {
            let n = r.tr.get("en")?.get("name")?.clone();
            Some((r.code, n))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_parsing() {
        let (t, id) = parse_catalog_record("skill:['basic', 'sword_a']").unwrap();
        assert_eq!(
            (t, id.pack.as_str(), id.sid.as_str()),
            ("skill", "basic", "sword_a")
        );
        assert_eq!(record_code("ref_attribute:DX"), Some("DX"));
        assert_eq!(record_code("pack:⟨basic⟩"), Some("basic"));
        assert_eq!(record_code("skill:['a', 'b']"), None);
        assert!(parse_catalog_record("nonsense").is_none());
    }
}
