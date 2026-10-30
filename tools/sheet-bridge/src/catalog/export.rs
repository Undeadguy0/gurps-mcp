//! export-catalog: паки БД -> плагины gurps-sheet (TOML + .ftl).

use super::{items, modifiers, skills, spells, traits};
use crate::db::Db;
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::plugin::{Locales, Manifest, Plugin};
use crate::sheet::DataFile;
use crate::sql::Rec;
use serde_json::Value as J;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// Запрос «собранных» записей таблицы (см. catalog/mod.rs) с фильтром.
pub fn assembled_query(table: &str, filter: &str) -> String {
    format!(
        "SELECT *, \
         (SELECT lang, field, text FROM translation WHERE record = $parent.id) AS _trl, \
         (SELECT attribute, mod FROM attr_default WHERE skill = $parent.id ORDER BY attribute) AS _attr_defaults, \
         (SELECT out, mod, any_spec, spec, from_spec, cap FROM defaults_to WHERE in = $parent.id ORDER BY out) AS _defaults_to \
         FROM {table} WHERE {filter} ORDER BY sid;"
    )
}

/// `_trl` (список) -> `_tr` (lang -> field -> text).
pub fn fold_tr(mut j: J) -> J {
    let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for t in jsonx::arr(&j, "_trl") {
        if let (Some(l), Some(f), Some(x)) = (jsonx::s(t, "lang"), jsonx::s(t, "field"), jsonx::s(t, "text")) {
            m.entry(l.into()).or_default().insert(f.into(), x.into());
        }
    }
    if let Some(o) = j.as_object_mut() {
        o.remove("_trl");
        o.insert("_tr".into(), serde_json::to_value(m).unwrap());
    }
    j
}

pub async fn fetch(db: &Db, table: &str, filter: &str) -> anyhow::Result<Vec<J>> {
    let v = db.one(&assembled_query(table, filter)).await?;
    Ok(v.as_array().cloned().unwrap_or_default().into_iter().map(fold_tr).collect())
}

pub async fn college_names(db: &Db) -> anyhow::Result<BTreeMap<String, String>> {
    let v = db
        .one("SELECT code, (SELECT VALUE text FROM translation WHERE record = $parent.id AND lang = 'en' AND field = 'name')[0] AS name FROM ref_college;")
        .await?;
    Ok(v.as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| Some((jsonx::s(r, "code")?.to_string(), jsonx::s(r, "name")?.to_string())))
        .collect())
}

/// Все записи пака по таблицам.
pub struct PackRows {
    pub rows: BTreeMap<&'static str, Vec<J>>,
}

pub async fn fetch_pack(db: &Db, pack: &str) -> anyhow::Result<PackRows> {
    let mut rows = BTreeMap::new();
    for t in super::TABLES {
        rows.insert(t, fetch(db, t, &format!("pack = pack:{pack}")).await?);
    }
    Ok(PackRows { rows })
}

fn add_loc(loc: &mut Locales, key: String, j: &J) {
    for (lang, e) in super::ftl_entries(j) {
        loc.entry(lang).or_default().insert(key.clone(), e);
    }
}

/// Записи -> DataFile + не-en переводы.
pub fn to_sheet(rows: &PackRows, report: &mut Report, colleges: &BTreeMap<String, String>) -> (DataFile, Locales) {
    let mut d = DataFile::default();
    let mut loc: Locales = BTreeMap::new();
    let get = |t: &str| rows.rows.get(t).map(Vec::as_slice).unwrap_or(&[]);
    for j in get("skill") {
        let x = skills::from_db(j, report);
        add_loc(&mut loc, format!("skill-{}", x.id), j);
        d.skills.push(x);
    }
    for j in get("technique") {
        let x = skills::tech_from_db(j, report);
        add_loc(&mut loc, format!("technique-{}", x.id), j);
        d.techniques.push(x);
    }
    for j in get("trait") {
        let x = traits::from_db(j, report, colleges);
        add_loc(&mut loc, format!("trait-{}", x.id), j);
        d.traits.push(x);
    }
    for j in get("modifier") {
        let x = modifiers::from_db(j, report);
        add_loc(&mut loc, format!("modifier-{}", x.id), j);
        d.modifiers.push(x);
    }
    for j in get("spell") {
        let x = spells::from_db(j, report, colleges);
        add_loc(&mut loc, format!("spell-{}", x.id), j);
        d.spells.push(x);
    }
    for j in get("item") {
        let (e, w) = items::from_db(j, report);
        add_loc(&mut loc, format!("equipment-{}", e.id), j);
        d.equipment.push(e);
        d.weapons.extend(w);
    }
    for j in get("attack_mode") {
        let w = items::natural_from_db(j, report);
        add_loc(&mut loc, format!("weapon-{}", w.id), j);
        d.weapons.push(w);
    }
    for j in rows.rows.values().flatten() {
        if let Some(o) = j.get("_tr").and_then(|t| t.as_object()) {
            for (lang, f) in o {
                if f.as_object().is_some_and(|m| m.keys().any(|k| k == "notes")) {
                    report.gap(
                        Dir::Export,
                        "translation.notes: в листе нет",
                        format!("{lang}:{}", jsonx::s(j, "id").unwrap_or("")),
                    );
                }
            }
        }
    }
    (d, loc)
}

/// Паки, на записи которых ссылаются записи этого пака (для depends).
pub fn referenced_packs(rows: &PackRows, own: &str) -> BTreeSet<String> {
    fn walk(v: &J, out: &mut BTreeSet<String>) {
        match v {
            J::String(s) => {
                if let Some(r) = Rec::parse(s) {
                    if let Some((p, _)) = r.pack_sid() {
                        if super::TABLES.contains(&r.table.as_str()) {
                            out.insert(p.to_string());
                        }
                    }
                }
            }
            J::Array(a) => a.iter().for_each(|x| walk(x, out)),
            J::Object(o) => o.iter().filter(|(k, _)| k.as_str() != "id").for_each(|(_, x)| walk(x, out)),
            _ => {}
        }
    }
    let mut out = BTreeSet::new();
    rows.rows.values().flatten().for_each(|j| walk(j, &mut out));
    out.remove(own);
    out
}

pub async fn run(db: &Db, packs: &[String], out_dir: &Path) -> anyhow::Result<Report> {
    let mut report = Report::default();
    let colleges = college_names(db).await?;
    let packs: Vec<String> = if packs.is_empty() {
        let v = db.one("SELECT VALUE id FROM pack WHERE kind != 'custom' ORDER BY id;").await?;
        v.as_array()
            .into_iter()
            .flatten()
            .filter_map(|x| x.as_str().and_then(Rec::parse))
            .filter_map(|r| r.code_str().map(str::to_string))
            .collect()
    } else {
        packs.iter().map(|p| ids::kebab_to_sid(p).unwrap_or_else(|_| p.clone())).collect()
    };
    for pack in packs {
        let meta = db.one(&format!("SELECT * FROM ONLY pack:{pack};")).await?;
        if meta.is_null() {
            anyhow::bail!("пак pack:{pack} не найден");
        }
        let rows = fetch_pack(db, &pack).await?;
        let (data, locales) = to_sheet(&rows, &mut report, &colleges);
        let depends: Vec<String> = referenced_packs(&rows, &pack).into_iter().map(|p| ids::sid_to_kebab(&p)).collect();
        let id = ids::sid_to_kebab(&pack);
        for k in ["abbrev", "kind", "priority", "spell_system", "notes"] {
            if meta.get(k).is_some_and(|v| !v.is_null()) {
                report.gap(Dir::Export, format!("pack.{k}: в plugin.toml нет"), id.clone());
            }
        }
        report.gap(Dir::Export, "plugin.version: в БД нет, записано 0.0.0", id.clone());
        report.gap(Dir::Export, "порядок записей: в БД нет, выводится по sid", id.clone());
        let manifest = Manifest {
            id: id.clone(),
            name: jsonx::s(&meta, "name").unwrap_or(&id).to_string(),
            version: "0.0.0".into(),
            depends,
        };
        let plugin = Plugin {
            manifest,
            data,
            locales,
            path: out_dir.join(&id),
        };
        let dir = crate::sheet::plugin::write_plugin(out_dir, &plugin)?;
        report.info(format!("pack:{pack} -> {}", dir.display()));
    }
    Ok(report)
}
