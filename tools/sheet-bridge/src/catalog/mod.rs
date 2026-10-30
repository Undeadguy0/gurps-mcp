//! Каталог: определения листа <-> записи SurrealDB.
//!
//! Каждая запись проходит через «собранный» JSON (`Assembled`): поля записи как их отдаёт
//! сервер плюс служебные `_tr` (переводы), `_attr_defaults`, `_defaults_to`. Импорт строит
//! его из определения листа, экспорт — из запросов к БД; `from_db` одинаков для обоих, что
//! даёт проекцию «как лист увидит это после БД» (нужна при сопоставлении персонажа).

pub mod damage;
pub mod export;
pub mod features;
pub mod import;
pub mod items;
pub mod modifiers;
pub mod resolve;
pub mod skills;
pub mod spells;
pub mod traits;

use crate::ids;
use crate::report::{Dir, Report};
use crate::sheet::{ftl::LocEntry, LocalizedText};
use crate::sql::{Rec, Sv};
use serde_json::Value as J;
use std::collections::BTreeMap;

pub use resolve::Resolver;

/// Таблицы каталога в порядке записи.
pub const TABLES: [&str; 7] = ["skill", "technique", "modifier", "trait", "attack_mode", "item", "spell"];

#[derive(Debug, Clone, PartialEq)]
pub struct Tr {
    pub lang: String,
    pub field: String,
    pub text: String,
}

/// Одна каталожная запись, готовая к записи.
#[derive(Debug, Clone)]
pub struct Entity {
    pub rec: Rec,
    pub content: Sv,
    pub tr: Vec<Tr>,
    /// (ref_attribute, mod) — строки `attr_default`
    pub attr_defaults: Vec<(Rec, i64)>,
    /// (skill, mod) — рёбра `defaults_to` с any_spec = true
    pub defaults_to: Vec<(Rec, i64)>,
}

impl Entity {
    pub fn new(rec: Rec, content: Sv) -> Self {
        Entity {
            rec,
            content,
            tr: vec![],
            attr_defaults: vec![],
            defaults_to: vec![],
        }
    }

    /// JSON в форме экспорта (см. модульный комментарий).
    pub fn assembled(&self) -> J {
        let mut j = self.content.to_json();
        let o = j.as_object_mut().expect("content is object");
        o.insert("id".into(), J::String(self.rec.to_string()));
        if let Some((p, s)) = self.rec.pack_sid() {
            o.insert("sid".into(), J::String(s.into()));
            o.insert("pack".into(), J::String(Rec::code("pack", p).to_string()));
        }
        o.insert("_tr".into(), tr_json(&self.tr));
        o.insert(
            "_attr_defaults".into(),
            J::Array(
                self.attr_defaults
                    .iter()
                    .map(|(a, m)| serde_json::json!({"attribute": a.to_string(), "mod": m}))
                    .collect(),
            ),
        );
        o.insert(
            "_defaults_to".into(),
            J::Array(
                self.defaults_to
                    .iter()
                    .map(|(s, m)| serde_json::json!({"out": s.to_string(), "mod": m, "any_spec": true}))
                    .collect(),
            ),
        );
        j
    }
}

pub fn tr_json(tr: &[Tr]) -> J {
    let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for t in tr {
        m.entry(t.lang.clone()).or_default().insert(t.field.clone(), t.text.clone());
    }
    serde_json::to_value(m).unwrap()
}

/// Контекст импорта одного пака.
pub struct Ctx<'a> {
    pub pack: String,
    pub resolver: &'a Resolver,
    pub report: &'a mut Report,
    /// Переводы из .ftl: lang -> key -> entry (уже слитые с языковыми паками).
    pub locales: &'a BTreeMap<String, BTreeMap<String, LocEntry>>,
    pub spell_class: String,
}

impl Ctx<'_> {
    pub fn rec(&self, table: &str, id: &str) -> anyhow::Result<Rec> {
        Ok(Rec::cat(table, &self.pack, &ids::kebab_to_sid(id)?))
    }
    /// Ссылка на запись каталога по kebab id (сначала текущий пак).
    pub fn find(&self, table: &str, id: &str) -> Option<Rec> {
        let sid = ids::kebab_to_sid(id).ok()?;
        self.resolver.find(table, &sid, &self.pack)
    }
    pub fn gap(&mut self, topic: &str, what: impl Into<String>) {
        self.report.gap(Dir::Import, topic, what);
    }

    /// Переводы name/description: TOML (en или локализованный description) + .ftl.
    pub fn texts(&self, kind: &str, id: &str, name: &str, desc: Option<&LocalizedText>) -> Vec<Tr> {
        let mut m: BTreeMap<(String, String), String> = BTreeMap::new();
        if !name.trim().is_empty() {
            m.insert(("en".into(), "name".into()), name.to_string());
        }
        for (lang, text) in desc.map(|d| d.by_lang()).unwrap_or_default() {
            if !text.trim().is_empty() {
                m.insert((lang, "description".into()), text.trim_end().to_string());
            }
        }
        let key = format!("{kind}-{id}");
        for (lang, entries) in self.locales {
            if let Some(e) = entries.get(&key) {
                if let Some(n) = &e.name {
                    m.insert((lang.clone(), "name".into()), n.clone());
                }
                if let Some(d) = &e.description {
                    m.insert((lang.clone(), "description".into()), d.clone());
                }
            }
        }
        m.into_iter().map(|((lang, field), text)| Tr { lang, field, text }).collect()
    }
}

/// Имя/описание из `_tr` собранного JSON (en; иначе любой язык; иначе sid).
pub fn tr_get(j: &J, lang: &str, field: &str) -> Option<String> {
    j.get("_tr")?.get(lang)?.get(field)?.as_str().map(str::to_string)
}

pub fn name_of(j: &J, report: &mut Report, what: &str) -> String {
    if let Some(n) = tr_get(j, "en", "name") {
        return n;
    }
    if let Some(n) = j
        .get("_tr")
        .and_then(|t| t.as_object())
        .and_then(|o| o.values().find_map(|l| l.get("name").and_then(|x| x.as_str()).map(str::to_string)))
    {
        report.gap(Dir::Export, "name: нет перевода en, взят другой язык", what);
        return n;
    }
    report.gap(Dir::Export, "name: нет перевода, взят sid", what);
    j.get("sid").and_then(|s| s.as_str()).unwrap_or("?").to_string()
}

pub fn desc_of(j: &J) -> Option<LocalizedText> {
    tr_get(j, "en", "description").map(|d| {
        let mut d = d;
        if !d.ends_with('\n') {
            d.push('\n');
        }
        LocalizedText::Plain(d)
    })
}

/// Переводы не-en языков для .ftl: lang -> entry.
pub fn ftl_entries(j: &J) -> BTreeMap<String, LocEntry> {
    let mut out = BTreeMap::new();
    if let Some(o) = j.get("_tr").and_then(|t| t.as_object()) {
        for (lang, fields) in o {
            if lang == "en" {
                continue;
            }
            let e = LocEntry {
                name: fields.get("name").and_then(|x| x.as_str()).map(str::to_string),
                description: fields.get("description").and_then(|x| x.as_str()).map(str::to_string),
            };
            if e.name.is_some() || e.description.is_some() {
                out.insert(lang.clone(), e);
            }
        }
    }
    out
}

pub fn kebab_sid(j: &J) -> String {
    ids::sid_to_kebab(j.get("sid").and_then(|s| s.as_str()).unwrap_or(""))
}

pub fn page_of(j: &J) -> Option<String> {
    ids::page_from_json(j)
}

/// Общие поля импорта: source_refs по `page` (с пометкой о потере нестандартных значений).
pub fn set_page(ctx: &mut Ctx, content: &mut Sv, page: Option<&str>, what: &str) {
    match page {
        None => {}
        Some(p) => match ids::source_refs(Some(p)) {
            Some(v) => {
                content.set("source_refs", v);
            }
            None => ctx.gap("page: не разбирается как <книга><страница>, отброшено", format!("{what} ({p:?})")),
        },
    }
}

/// Число листа -> Sv (целое, если без дробной части).
pub fn num(x: f64) -> Sv {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        Sv::Int(x as i64)
    } else {
        Sv::Float(x)
    }
}
