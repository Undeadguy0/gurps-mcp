//! Идентификаторы: kebab-case id листа <-> sid БД (`^[a-z0-9_]+$`), ссылки на книги.

use crate::sql::{Rec, Sv};

/// `combat-reflexes` -> `combat_reflexes`. Ошибка, если id не обратим однозначно.
pub fn kebab_to_sid(id: &str) -> anyhow::Result<String> {
    if id.is_empty() {
        anyhow::bail!("пустой id");
    }
    if !id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        anyhow::bail!("id {id:?}: допустимы только [a-z0-9-] (kebab-case), иначе sid необратим");
    }
    Ok(id.replace('-', "_"))
}

pub fn sid_to_kebab(sid: &str) -> String {
    sid.replace('_', "-")
}

/// Свободный текст -> sid (`Island/Beach` -> `island_beach`).
pub fn slug(s: &str) -> String {
    let mut out = String::new();
    let mut gap = false;
    for c in s.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            if gap && !out.is_empty() {
                out.push('_');
            }
            gap = false;
            out.push(c);
        } else {
            gap = true;
        }
    }
    out
}

/// `B208` -> `{book: ref_book:B, page: 208}`; иное (`example`) не представимо.
pub fn parse_page(page: &str) -> Option<(String, i64)> {
    let p = page.trim();
    let split = p.find(|c: char| c.is_ascii_digit())?;
    let (book, num) = p.split_at(split);
    if book.is_empty() || !book.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let n: i64 = num.parse().ok()?;
    Some((book.to_string(), n))
}

pub fn source_refs(page: Option<&str>) -> Option<Sv> {
    let (book, n) = parse_page(page?)?;
    Some(Sv::Arr(vec![Sv::obj()
        .with("book", Sv::Rec(Rec::code("ref_book", &book)))
        .with("page", Sv::Int(n))]))
}

/// Обратно: первая ссылка `source_refs` -> `B208`.
pub fn page_from_json(v: &serde_json::Value) -> Option<String> {
    let r = v.get("source_refs")?.as_array()?.first()?;
    let raw = r.get("book")?.as_str()?;
    let rec = Rec::parse(raw)?;
    if rec.table != "ref_book" {
        return None;
    }
    let book = rec.code_str()?;
    match r.get("page").and_then(|p| p.as_i64()) {
        Some(n) => Some(format!("{book}{n}")),
        None => Some(book.to_string()),
    }
}

/// Код атрибута листа (`Stat::key`) -> код `ref_attribute`.
pub fn stat_to_ref(stat: &str) -> String {
    match stat {
        "BasicSpeed" => "Speed".into(),
        "BasicMove" => "Move".into(),
        s => s.into(),
    }
}

pub fn ref_to_stat(code: &str) -> String {
    match code {
        "Speed" => "BasicSpeed".into(),
        "Move" => "BasicMove".into(),
        s => s.into(),
    }
}
