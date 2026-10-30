//! Fluent (.ftl): чтение по правилам gurps-plugins (`<kind>-<id>` = имя, `.description`)
//! и запись с экранированием спецсимволов.

use fluent_syntax::ast::{Entry, Expression, InlineExpression, Pattern, PatternElement};
use std::collections::BTreeMap;
use std::fmt::Write;

/// Виды записей в ключах сообщений (как `gurps_plugins::l10n::KINDS`).
pub const KINDS: [&str; 7] = ["trait", "skill", "technique", "spell", "equipment", "weapon", "modifier"];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocEntry {
    pub name: Option<String>,
    pub description: Option<String>,
}

fn pattern_text(p: &Pattern<&str>) -> String {
    let mut s = String::new();
    for el in &p.elements {
        match el {
            PatternElement::TextElement { value } => s.push_str(value),
            PatternElement::Placeable { expression } => match expression {
                Expression::Inline(InlineExpression::StringLiteral { value }) => {
                    s.push_str(&value.replace("\\\"", "\"").replace("\\\\", "\\"))
                }
                Expression::Inline(InlineExpression::NumberLiteral { value }) => s.push_str(value),
                _ => {}
            },
        }
    }
    s.trim_end().to_string()
}

pub fn parse_ftl(src: &str, path: &str) -> anyhow::Result<BTreeMap<String, LocEntry>> {
    let res = fluent_syntax::parser::parse(src).map_err(|(_, errs)| {
        anyhow::anyhow!(
            "ошибка разбора Fluent в {path}: {}",
            errs.iter()
                .map(|e| format!("{:?} at {:?}", e.kind, e.pos))
                .collect::<Vec<_>>()
                .join("; ")
        )
    })?;
    let mut out = BTreeMap::new();
    for e in &res.body {
        if let Entry::Message(m) = e {
            let name = m.value.as_ref().map(pattern_text).filter(|t| !t.is_empty());
            let description = m
                .attributes
                .iter()
                .find(|a| a.id.name == "description")
                .map(|a| pattern_text(&a.value))
                .filter(|t| !t.trim().is_empty());
            out.insert(m.id.name.to_string(), LocEntry { name, description });
        }
    }
    Ok(out)
}

/// Делит ключ `<kind>-<id>` на вид и id.
pub fn split_key(key: &str) -> Option<(&'static str, &str)> {
    KINDS
        .iter()
        .find_map(|k| key.strip_prefix(k).and_then(|r| r.strip_prefix('-')).map(|id| (*k, id)))
}

fn escape_line(line: &str) -> String {
    let mut out = String::new();
    for (i, c) in line.chars().enumerate() {
        match c {
            '{' => out.push_str("{\"{\"}"),
            '}' => out.push_str("{\"}\"}"),
            '[' | '*' | '.' if i == 0 => {
                let _ = write!(out, "{{\"{c}\"}}");
            }
            c => out.push(c),
        }
    }
    out
}

/// Сериализует сообщения; порядок — как в `entries`.
pub fn write_ftl(header: &str, entries: &[(String, LocEntry)]) -> String {
    let mut s = String::new();
    for l in header.lines() {
        let _ = writeln!(s, "# {l}");
    }
    for (key, e) in entries {
        s.push('\n');
        let name = e.name.as_deref().map(|n| escape_line(n.trim())).unwrap_or_default();
        if name.is_empty() {
            let _ = writeln!(s, "{key} =");
        } else {
            let _ = writeln!(s, "{key} = {name}");
        }
        if let Some(d) = e.description.as_deref().filter(|d| !d.trim().is_empty()) {
            s.push_str("    .description =\n");
            for line in d.trim_end().lines() {
                // относительный отступ сохраняется: Fluent снимает только общий
                let t = line.trim_end();
                if t.trim().is_empty() {
                    s.push('\n');
                } else {
                    let body = t.trim_start();
                    let indent = &t[..t.len() - body.len()];
                    // отступ в строковом литерале: парсер Fluent иначе его съедает
                    let lead = if indent.is_empty() {
                        String::new()
                    } else {
                        format!("{{\"{indent}\"}}")
                    };
                    let _ = writeln!(s, "        {lead}{}", escape_line(body));
                }
            }
        }
    }
    s
}
