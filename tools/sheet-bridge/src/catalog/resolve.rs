//! Поиск каталожных записей по sid: сначала текущий пак, затем пак с наибольшим priority.

use crate::db::Db;
use crate::jsonx;
use crate::sql::Rec;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Resolver {
    /// (table, sid) -> [(pack, priority)]
    map: BTreeMap<(String, String), Vec<(String, i64)>>,
    pub priorities: BTreeMap<String, i64>,
    /// Имя школы (en из translation, в нижнем регистре) и код -> код ref_college.
    pub colleges: BTreeMap<String, String>,
}

impl Resolver {
    pub fn add(&mut self, table: &str, pack: &str, sid: &str, priority: i64) {
        let v = self.map.entry((table.into(), sid.into())).or_default();
        if !v.iter().any(|(p, _)| p == pack) {
            v.push((pack.into(), priority));
        }
        self.priorities.entry(pack.into()).or_insert(priority);
    }

    pub fn find(&self, table: &str, sid: &str, prefer: &str) -> Option<Rec> {
        let v = self.map.get(&(table.to_string(), sid.to_string()))?;
        let pack = v
            .iter()
            .find(|(p, _)| p == prefer)
            .or_else(|| v.iter().max_by_key(|(p, prio)| (*prio, std::cmp::Reverse(p.clone()))))?;
        Some(Rec::cat(table, &pack.0, sid))
    }

    /// Паки (кроме `except`), где есть запись с таким sid.
    pub fn packs_with(&self, table: &str, sid: &str) -> Vec<String> {
        self.map
            .get(&(table.to_string(), sid.to_string()))
            .map(|v| v.iter().map(|(p, _)| p.clone()).collect())
            .unwrap_or_default()
    }

    /// Все каталожные id из БД (паки kind = custom пропускаются, кроме `include_custom`).
    pub async fn load(db: &Db, include_custom: &[String]) -> anyhow::Result<Self> {
        let mut r = Resolver::default();
        let packs = db.one("SELECT id, priority, kind FROM pack;").await?;
        let mut kinds = BTreeMap::new();
        for p in packs.as_array().into_iter().flatten() {
            if let Some(id) = jsonx::rec(p, "id").and_then(|r| r.code_str().map(str::to_string)) {
                r.priorities.insert(id.clone(), jsonx::i(p, "priority").unwrap_or(0));
                kinds.insert(id, jsonx::s(p, "kind").unwrap_or("").to_string());
            }
        }
        for t in super::TABLES {
            let rows = db.one(&format!("SELECT VALUE id FROM {t};")).await?;
            for v in rows.as_array().into_iter().flatten() {
                let Some(rec) = v.as_str().and_then(Rec::parse) else { continue };
                let Some((p, s)) = rec.pack_sid() else { continue };
                if kinds.get(p).map(String::as_str) == Some("custom") && !include_custom.iter().any(|c| c == p) {
                    continue;
                }
                let prio = r.priorities.get(p).copied().unwrap_or(0);
                r.add(t, p, s, prio);
            }
        }
        for (code, name) in super::export::college_names(db).await? {
            r.colleges.insert(code.to_lowercase(), code.clone());
            r.colleges.insert(name.to_lowercase(), code);
        }
        Ok(r)
    }

    /// Код ref_college по имени из листа: сначала имя/код из БД, иначе slug имени.
    pub fn college_code(&self, name: &str) -> String {
        self.colleges
            .get(&name.trim().to_lowercase())
            .cloned()
            .unwrap_or_else(|| crate::ids::slug(name))
    }
}
