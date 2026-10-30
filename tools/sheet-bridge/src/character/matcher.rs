//! Сопоставление встроенных в персонажа определений с каталогом.
//!
//! Определение листа проецируется через схему (to_db -> from_db) и сравнивается с записью
//! каталога, экспортированной в форму листа (описание не сравнивается). Совпало — ребро ведёт
//! на запись каталога; нет — определение копируется в пак персонажа `custom_<id>`.

use crate::catalog::{export, Ctx, Entity, Resolver};
use crate::db::Db;
use crate::ids;
use crate::report::{Dir, Report};
use crate::sql::Rec;
use serde::Serialize;
use serde_json::Value as J;
use std::collections::BTreeMap;

pub struct Matcher<'a> {
    pub db: &'a Db,
    pub resolver: Resolver,
    pub pack: String,
    pub spell_class: String,
    pub colleges: BTreeMap<String, String>,
    pub customs: Vec<Entity>,
    pub placed: BTreeMap<(String, String), (Rec, J)>,
    pub report: &'a mut Report,
}

fn strip(mut v: J) -> J {
    if let Some(o) = v.as_object_mut() {
        o.remove("description");
    }
    v
}

impl Matcher<'_> {
    /// Куда вести ребро для определения `def` (таблица `table`, sid `sid`).
    #[allow(clippy::too_many_arguments)]
    pub async fn place<D: Serialize>(
        &mut self,
        table: &str,
        sid: &str,
        source: Option<&str>,
        def: &D,
        to_db: impl Fn(&mut Ctx, &D) -> anyhow::Result<Entity>,
        from_db: impl Fn(&J, &mut Report, &BTreeMap<String, String>) -> J,
        what: &str,
    ) -> anyhow::Result<Rec> {
        let mine = serde_json::to_value(def)?;
        let key = (table.to_string(), sid.to_string());
        if let Some((rec, prev)) = self.placed.get(&key) {
            if strip(prev.clone()) != strip(mine.clone()) {
                self.report
                    .gap(Dir::Import, "одинаковый id с разным содержимым: использовано первое", what);
            }
            return Ok(rec.clone());
        }
        let empty = BTreeMap::new();
        let mut scratch = Report::default();
        // проекция
        let proj = {
            let mut ctx = Ctx {
                pack: self.pack.clone(),
                resolver: &self.resolver,
                report: &mut scratch,
                locales: &empty,
                spell_class: self.spell_class.clone(),
            };
            let ent = to_db(&mut ctx, def)?;
            strip(from_db(&ent.assembled(), &mut scratch, &self.colleges))
        };
        // потери проекции (лист -> БД) относятся и к ребру на каталог
        for ((d, t), ws) in &scratch.gaps {
            if *d == Dir::Import {
                for w in ws {
                    self.report.gap(Dir::Import, t.clone(), w.clone());
                }
            }
        }
        let prefer = source.and_then(|s| ids::kebab_to_sid(s).ok()).unwrap_or_default();
        if let Some(cat) = self.resolver.find(table, sid, &prefer) {
            let rows = export::fetch(self.db, table, &format!("id = {cat}")).await?;
            if let Some(j) = rows.first() {
                let cat_def = strip(from_db(j, &mut scratch, &self.colleges));
                if cat_def == proj {
                    self.placed.insert(key, (cat.clone(), mine));
                    return Ok(cat);
                }
                self.report.info(format!("{what}: отличается от {cat} -> пак {}", self.pack));
            }
        } else {
            self.report.info(format!("{what}: нет в каталоге -> пак {}", self.pack));
        }
        let mut ctx = Ctx {
            pack: self.pack.clone(),
            resolver: &self.resolver,
            report: self.report,
            locales: &empty,
            spell_class: self.spell_class.clone(),
        };
        let ent = to_db(&mut ctx, def)?;
        let rec = ent.rec.clone();
        self.resolver.add(table, &self.pack, sid, 1000);
        self.customs.push(ent);
        self.placed.insert(key, (rec.clone(), mine));
        Ok(rec)
    }
}
