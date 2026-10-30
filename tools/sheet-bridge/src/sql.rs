//! Значения SurrealQL и их текстовая запись (литералы) для генерируемых запросов.
//! Ссылки на записи — типизированные (`Rec`), а не строки: схема SCHEMAFULL их не приводит.

use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// `ref_attribute:DX`, `character:warrior`
    Str(String),
    /// `skill:['pack', 'sid']`, `owns_item:['warrior', 0, 1]`
    Arr(Vec<KeyPart>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyPart {
    S(String),
    I(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rec {
    pub table: String,
    pub key: Key,
}

impl Rec {
    pub fn code(table: &str, code: &str) -> Self {
        Rec {
            table: table.into(),
            key: Key::Str(code.into()),
        }
    }
    pub fn cat(table: &str, pack: &str, sid: &str) -> Self {
        Rec {
            table: table.into(),
            key: Key::Arr(vec![KeyPart::S(pack.into()), KeyPart::S(sid.into())]),
        }
    }
    /// (pack, sid) каталожной записи.
    pub fn pack_sid(&self) -> Option<(&str, &str)> {
        match &self.key {
            Key::Arr(v) => match v.as_slice() {
                [KeyPart::S(p), KeyPart::S(s)] => Some((p, s)),
                _ => None,
            },
            _ => None,
        }
    }
    pub fn code_str(&self) -> Option<&str> {
        match &self.key {
            Key::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Разбор `table:key` в форме, которую возвращает сервер (`RecordId::to_sql`).
    pub fn parse(s: &str) -> Option<Rec> {
        let (table, key) = s.split_once(':')?;
        if table.is_empty() || !table.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return None;
        }
        let key = key.trim();
        let key = if let Some(inner) = key.strip_prefix('[').and_then(|k| k.strip_suffix(']')) {
            let mut parts = Vec::new();
            for raw in split_top(inner) {
                let t = raw.trim();
                if t.is_empty() {
                    continue;
                }
                if let Some(q) = unquote(t) {
                    parts.push(KeyPart::S(q));
                } else {
                    parts.push(KeyPart::I(t.parse().ok()?));
                }
            }
            Key::Arr(parts)
        } else if let Some(q) = key.strip_prefix('⟨').and_then(|k| k.strip_suffix('⟩')) {
            Key::Str(q.replace("\\⟩", "⟩"))
        } else if let Some(q) = key.strip_prefix('`').and_then(|k| k.strip_suffix('`')) {
            Key::Str(q.to_string())
        } else {
            Key::Str(key.to_string())
        };
        Some(Rec { table: table.into(), key })
    }
}

fn split_top(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut esc = false;
    for c in s.chars() {
        if let Some(q) = quote {
            cur.push(c);
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == q {
                quote = None;
            }
            continue;
        }
        match c {
            '\'' | '"' => {
                quote = Some(c);
                cur.push(c);
            }
            ',' => out.push(std::mem::take(&mut cur)),
            c => cur.push(c),
        }
    }
    out.push(cur);
    out
}

fn unquote(t: &str) -> Option<String> {
    let q = t.chars().next()?;
    if (q == '\'' || q == '"') && t.ends_with(q) && t.len() >= 2 {
        let inner = &t[1..t.len() - 1];
        let mut out = String::new();
        let mut esc = false;
        for c in inner.chars() {
            if esc {
                out.push(match c {
                    'n' => '\n',
                    't' => '\t',
                    c => c,
                });
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else {
                out.push(c);
            }
        }
        Some(out)
    } else {
        None
    }
}

impl std::fmt::Display for Rec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.table)?;
        match &self.key {
            Key::Str(s)
                if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && !s.chars().all(|c| c.is_ascii_digit()) =>
            {
                f.write_str(s)
            }
            Key::Str(s) => write!(f, "⟨{}⟩", s.replace('⟩', "\\⟩")),
            Key::Arr(v) => {
                f.write_str("[")?;
                for (i, p) in v.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    match p {
                        KeyPart::S(s) => f.write_str(&quote(s))?,
                        KeyPart::I(n) => write!(f, "{n}")?,
                    }
                }
                f.write_str("]")
            }
        }
    }
}

pub fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}

/// Значение для литерала SurrealQL.
#[derive(Debug, Clone, PartialEq)]
pub enum Sv {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Rec(Rec),
    Arr(Vec<Sv>),
    Obj(BTreeMap<String, Sv>),
    /// Готовое выражение (например `d'2026-01-01T00:00:00Z'`).
    Raw(String),
}

impl Sv {
    pub fn obj() -> Self {
        Sv::Obj(BTreeMap::new())
    }
    pub fn str(s: impl Into<String>) -> Self {
        Sv::Str(s.into())
    }
    /// Добавить поле (None пропускается).
    pub fn set(&mut self, k: &str, v: Sv) -> &mut Self {
        if let Sv::Obj(m) = self {
            if v != Sv::None {
                m.insert(k.into(), v);
            }
        }
        self
    }
    pub fn with(mut self, k: &str, v: Sv) -> Self {
        self.set(k, v);
        self
    }
    pub fn opt<T>(v: Option<T>, f: impl FnOnce(T) -> Sv) -> Sv {
        v.map(f).unwrap_or(Sv::None)
    }

    pub fn write(&self, out: &mut String) {
        match self {
            Sv::None => out.push_str("NONE"),
            Sv::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Sv::Int(n) => {
                let _ = write!(out, "{n}");
            }
            Sv::Float(x) => {
                if x.fract() == 0.0 && x.abs() < 1e15 {
                    let _ = write!(out, "{x:.1}f");
                } else {
                    let _ = write!(out, "{x}f");
                }
            }
            Sv::Str(s) => out.push_str(&quote(s)),
            Sv::Rec(r) => {
                let _ = write!(out, "{r}");
            }
            Sv::Raw(s) => out.push_str(s),
            Sv::Arr(v) => {
                out.push('[');
                for (i, x) in v.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    x.write(out);
                }
                out.push(']');
            }
            Sv::Obj(m) => {
                out.push('{');
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    let _ = write!(out, "{}: ", quote(k));
                    v.write(out);
                }
                out.push('}');
            }
        }
    }

    pub fn to_sql(&self) -> String {
        let mut s = String::new();
        self.write(&mut s);
        s
    }

    /// JSON в том виде, в каком сервер отдаёт запись (ссылки — строки `table:key`).
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::Value as J;
        match self {
            Sv::None => J::Null,
            Sv::Bool(b) => J::Bool(*b),
            Sv::Int(n) => J::from(*n),
            Sv::Float(x) => serde_json::Number::from_f64(*x).map(J::Number).unwrap_or(J::Null),
            Sv::Str(s) | Sv::Raw(s) => J::String(s.clone()),
            Sv::Rec(r) => J::String(r.to_string()),
            Sv::Arr(v) => J::Array(v.iter().map(Sv::to_json).collect()),
            Sv::Obj(m) => J::Object(m.iter().map(|(k, v)| (k.clone(), v.to_json())).collect()),
        }
    }

    /// Все ссылки внутри значения.
    pub fn collect_recs(&self, out: &mut Vec<Rec>) {
        match self {
            Sv::Rec(r) => out.push(r.clone()),
            Sv::Arr(v) => v.iter().for_each(|x| x.collect_recs(out)),
            Sv::Obj(m) => m.values().for_each(|x| x.collect_recs(out)),
            _ => {}
        }
    }
}
