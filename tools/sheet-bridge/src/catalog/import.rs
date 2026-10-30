//! import-catalog: плагины (TOML + .ftl) -> паки и записи БД.

use super::{items, modifiers, skills, spells, traits, Ctx, Entity, Resolver, Tr};
use crate::db::Db;
use crate::ids;
use crate::report::{Dir, Report};
use crate::sheet::ftl::{split_key, LocEntry};
use crate::sheet::plugin::{load_plugins, Plugin};
use crate::sql::{Rec, Sv};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CatalogOpts {
    pub kind: String,
    pub abbrev: Option<String>,
    pub spell_system: String,
    pub spell_class: String,
}

impl Default for CatalogOpts {
    fn default() -> Self {
        CatalogOpts {
            kind: "homebrew".into(),
            abbrev: None,
            spell_system: "standard".into(),
            spell_class: "regular".into(),
        }
    }
}

/// План записи одного пака.
pub struct PackPlan {
    pub pack: String,
    pub content: Sv,
    pub entities: Vec<Entity>,
}

/// Таблица БД для вида ключа .ftl.
pub fn kind_table(kind: &str) -> &'static str {
    match kind {
        "trait" => "trait",
        "skill" => "skill",
        "technique" => "technique",
        "spell" => "spell",
        "equipment" => "item",
        "weapon" => "attack_mode",
        _ => "modifier",
    }
}

/// Зарегистрировать будущие записи плагина в резолвере (до конвертации, ради перекрёстных ссылок).
pub fn register(res: &mut Resolver, p: &Plugin, pack: &str, prio: i64) -> anyhow::Result<()> {
    let d = &p.data;
    let mut add = |t: &str, id: &str| -> anyhow::Result<()> {
        res.add(
            t,
            pack,
            &ids::kebab_to_sid(id).map_err(|e| anyhow::anyhow!("{t} {id:?}: {e}"))?,
            prio,
        );
        Ok(())
    };
    d.skills.iter().try_for_each(|x| add("skill", &x.id))?;
    d.techniques.iter().try_for_each(|x| add("technique", &x.id))?;
    d.traits.iter().try_for_each(|x| add("trait", &x.id))?;
    d.modifiers.iter().try_for_each(|x| add("modifier", &x.id))?;
    d.spells.iter().try_for_each(|x| add("spell", &x.id))?;
    d.equipment.iter().try_for_each(|x| add("item", &x.id))?;
    d.weapons
        .iter()
        .filter(|w| w.equipment.is_none())
        .try_for_each(|x| add("attack_mode", &x.id))?;
    Ok(())
}

/// Записи данных плагина (без пака).
pub fn convert_data(ctx: &mut Ctx, p: &Plugin) -> anyhow::Result<Vec<Entity>> {
    let d = &p.data;
    let mut out = Vec::new();
    for x in &d.skills {
        out.push(skills::to_db(ctx, x)?);
    }
    for x in &d.techniques {
        out.push(skills::tech_to_db(ctx, x)?);
    }
    for x in &d.modifiers {
        out.push(modifiers::to_db(ctx, x)?);
    }
    for x in &d.traits {
        out.push(traits::to_db(ctx, x)?);
    }
    let eq_ids: BTreeSet<&str> = d.equipment.iter().map(|e| e.id.as_str()).collect();
    for w in &d.weapons {
        match &w.equipment {
            None => out.push(items::natural_to_db(ctx, w)?),
            Some(e) if !eq_ids.contains(e.as_str()) => {
                anyhow::bail!("weapon {}: equipment {e:?} нет в этом плагине", w.id)
            }
            _ => {}
        }
    }
    for e in &d.equipment {
        if e.id.is_empty() {
            anyhow::bail!("equipment {:?}: пустой id в каталоге", e.name);
        }
        let ws: Vec<_> = d.weapons.iter().filter(|w| w.equipment.as_deref() == Some(e.id.as_str())).collect();
        out.push(items::to_db(ctx, e, &ids::kebab_to_sid(&e.id)?, &ws)?);
    }
    for x in &d.spells {
        out.push(spells::to_db(ctx, x)?);
    }
    let mut seen = BTreeSet::new();
    for e in &out {
        if !seen.insert(e.rec.clone()) {
            anyhow::bail!("повторяющийся id {}", e.rec);
        }
    }
    // ключи .ftl без записи
    let have: BTreeSet<(String, String)> = out
        .iter()
        .filter_map(|e| e.rec.pack_sid().map(|(_, s)| (e.rec.table.clone(), s.to_string())))
        .collect();
    for (lang, entries) in ctx.locales.iter() {
        for key in entries.keys() {
            let ok = split_key(key)
                .map(|(k, id)| have.contains(&(kind_table(k).to_string(), id.replace('-', "_"))))
                .unwrap_or(false);
            if !ok && !weapon_key_linked(p, key) {
                ctx.report
                    .gap(Dir::Import, "ftl: ключ без записи, отброшен", format!("{lang}:{key}"));
            }
        }
    }
    Ok(out)
}

fn weapon_key_linked(p: &Plugin, key: &str) -> bool {
    key.strip_prefix("weapon-")
        .is_some_and(|id| p.data.weapons.iter().any(|w| w.id == id && w.equipment.is_some()))
}

/// Слить переводы плагина и языковых паков, зависящих от него.
pub fn merged_locales(p: &Plugin, all: &[Plugin]) -> BTreeMap<String, BTreeMap<String, LocEntry>> {
    let mut m = p.locales.clone();
    for lp in all
        .iter()
        .filter(|x| x.is_locale_only() && x.manifest.depends.contains(&p.manifest.id))
    {
        for (lang, entries) in &lp.locales {
            m.entry(lang.clone()).or_default().extend(entries.clone());
        }
    }
    m
}

/// Переводы языковых плагинов, привязанные к записям зависимостей.
pub type ExtraTr = Vec<(Rec, Tr)>;

pub fn plan(all: &[Plugin], resolver: &mut Resolver, report: &mut Report, opts: &CatalogOpts) -> anyhow::Result<(Vec<PackPlan>, ExtraTr)> {
    if opts.kind == "custom" {
        anyhow::bail!("--kind custom зарезервирован для паков персонажей (нужен owner)");
    }
    let data: Vec<&Plugin> = all.iter().filter(|p| !p.is_locale_only()).collect();
    // priority = глубина зависимостей * 10
    let mut depth: BTreeMap<String, i64> = BTreeMap::new();
    for p in &data {
        let d = p
            .manifest
            .depends
            .iter()
            .filter_map(|x| depth.get(x))
            .max()
            .map(|x| x + 1)
            .unwrap_or(0);
        depth.insert(p.manifest.id.clone(), d);
    }
    for p in &data {
        let pack = ids::kebab_to_sid(&p.manifest.id)?;
        register(resolver, p, &pack, depth[&p.manifest.id] * 10)?;
    }
    let mut plans = Vec::new();
    for p in &data {
        let pack = ids::kebab_to_sid(&p.manifest.id)?;
        let locales = merged_locales(p, all);
        let mut ctx = Ctx {
            pack: pack.clone(),
            resolver,
            report,
            locales: &locales,
            spell_class: opts.spell_class.clone(),
        };
        let entities = convert_data(&mut ctx, p)?;
        let mut content = Sv::obj()
            .with("name", Sv::str(&p.manifest.name))
            .with(
                "abbrev",
                Sv::str(opts.abbrev.clone().filter(|_| data.len() == 1).unwrap_or_else(|| pack.clone())),
            )
            .with("kind", Sv::str(&opts.kind))
            .with("priority", Sv::Int(depth[&p.manifest.id] * 10));
        if !p.data.spells.is_empty() {
            content.set("spell_system", Sv::Rec(Rec::code("ref_spell_system", &opts.spell_system)));
        }
        report.gap(Dir::Import, "plugin.version: в схеме пака нет", p.manifest.id.clone());
        if !p.manifest.depends.is_empty() {
            report.gap(
                Dir::Import,
                "plugin.depends: в схеме пака нет (порядок — через priority)",
                p.manifest.id.clone(),
            );
        }
        report.info(format!("пак {pack}: {} записей", entities.len()));
        plans.push(PackPlan { pack, content, entities });
    }
    // языковые паки, чьи зависимости уже в БД (не в этом импорте)
    let mut extra = Vec::new();
    for lp in all.iter().filter(|p| p.is_locale_only()) {
        report.gap(
            Dir::Import,
            "языковой плагин: не становится паком, переводы привязаны к записям зависимостей",
            lp.manifest.id.clone(),
        );
        for dep in &lp.manifest.depends {
            if data.iter().any(|p| &p.manifest.id == dep) {
                continue;
            }
            let dep_pack = ids::kebab_to_sid(dep)?;
            for (lang, entries) in &lp.locales {
                for (key, e) in entries {
                    let Some((k, id)) = split_key(key) else { continue };
                    let rec = resolver
                        .find(kind_table(k), &id.replace('-', "_"), &dep_pack)
                        .filter(|r| r.pack_sid().map(|x| x.0) == Some(dep_pack.as_str()));
                    let Some(rec) = rec else {
                        report.gap(Dir::Import, "ftl: ключ без записи, отброшен", format!("{lang}:{key}"));
                        continue;
                    };
                    for (field, text) in [("name", &e.name), ("description", &e.description)] {
                        if let Some(t) = text {
                            extra.push((
                                rec.clone(),
                                Tr {
                                    lang: lang.clone(),
                                    field: field.into(),
                                    text: t.clone(),
                                },
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok((plans, extra))
}

pub fn ref_checks(entities: &[Entity], out: &mut BTreeMap<Rec, String>) {
    for e in entities {
        let mut v = Vec::new();
        e.content.collect_recs(&mut v);
        v.extend(e.attr_defaults.iter().map(|(a, _)| a.clone()));
        for r in v.into_iter().filter(|r| r.table.starts_with("ref_")) {
            out.entry(r).or_insert_with(|| e.rec.to_string());
        }
    }
}

pub fn entity_sql(sql: &mut String, e: &Entity) {
    let _ = writeln!(sql, "UPSERT {} CONTENT {};", e.rec, e.content.to_sql());
}

pub fn edges_sql(sql: &mut String, e: &Entity) {
    if e.rec.table == "skill" {
        let _ = writeln!(sql, "DELETE attr_default WHERE skill = {};", e.rec);
    }
    if e.rec.table == "skill" || e.rec.table == "technique" {
        let _ = writeln!(sql, "DELETE defaults_to WHERE in = {};", e.rec);
    }
    for (a, m) in &e.attr_defaults {
        let c = Sv::obj()
            .with("skill", Sv::Rec(e.rec.clone()))
            .with("attribute", Sv::Rec(a.clone()))
            .with("mod", Sv::Int(*m));
        let _ = writeln!(sql, "CREATE attr_default CONTENT {};", c.to_sql());
    }
    for (s, m) in &e.defaults_to {
        let _ = writeln!(sql, "RELATE {}->defaults_to->{} CONTENT {{ mod: {m}, any_spec: true }};", e.rec, s);
    }
}

pub fn translations_sql(sql: &mut String, e: &Entity) {
    let _ = writeln!(sql, "DELETE translation WHERE record = {};", e.rec);
    for t in &e.tr {
        tr_sql(sql, &e.rec, t);
    }
}

pub fn tr_sql(sql: &mut String, rec: &Rec, t: &Tr) {
    let c = Sv::obj()
        .with("record", Sv::Rec(rec.clone()))
        .with("lang", Sv::str(&t.lang))
        .with("field", Sv::str(&t.field))
        .with("text", Sv::str(&t.text));
    let _ = writeln!(sql, "CREATE translation CONTENT {};", c.to_sql());
}

pub fn build_sql(plans: &[PackPlan], extra: &[(Rec, Tr)]) -> String {
    let mut sql = String::new();
    for p in plans {
        let _ = writeln!(sql, "UPSERT pack:{} MERGE {};", p.pack, p.content.to_sql());
    }
    let all: Vec<&Entity> = plans.iter().flat_map(|p| &p.entities).collect();
    for t in super::TABLES {
        for e in all.iter().filter(|e| e.rec.table == t) {
            entity_sql(&mut sql, e);
        }
    }
    for e in &all {
        edges_sql(&mut sql, e);
        translations_sql(&mut sql, e);
    }
    for (rec, t) in extra {
        let _ = writeln!(
            sql,
            "DELETE translation WHERE record = {rec} AND lang = {} AND field = {};",
            crate::sql::quote(&t.lang),
            crate::sql::quote(&t.field)
        );
        tr_sql(&mut sql, rec, t);
    }
    sql
}

/// Проверить существование справочников; ошибка со списком неизвестных кодов.
pub async fn check_refs(db: &Db, need: &BTreeMap<Rec, String>) -> anyhow::Result<()> {
    let recs: Vec<Rec> = need.keys().cloned().collect();
    let missing = db.missing(&recs).await?;
    if !missing.is_empty() {
        let list = missing
            .iter()
            .map(|r| format!("  {r} (нужен для {})", need[r]))
            .collect::<Vec<_>>()
            .join("\n");
        anyhow::bail!("неизвестные коды справочников (создаются только вручную/сидами, импорт их не создаёт):\n{list}");
    }
    Ok(())
}

pub async fn run(db: &Db, paths: &[PathBuf], opts: &CatalogOpts, dry_run: bool) -> anyhow::Result<Report> {
    let plugins = load_plugins(paths)?;
    if plugins.is_empty() {
        anyhow::bail!("плагины не найдены: {paths:?}");
    }
    let mut report = Report::default();
    let mut resolver = Resolver::load(db, &[]).await?;
    let (plans, extra) = plan(&plugins, &mut resolver, &mut report, opts)?;
    let mut need = BTreeMap::new();
    for p in &plans {
        ref_checks(&p.entities, &mut need);
        let mut v = Vec::new();
        p.content.collect_recs(&mut v);
        for r in v {
            need.insert(r, format!("pack:{}", p.pack));
        }
    }
    check_refs(db, &need).await?;
    db.write(&build_sql(&plans, &extra), dry_run).await?;
    report.info(if dry_run {
        "dry-run: транзакция отменена, БД не изменена".to_string()
    } else {
        "записано".to_string()
    });
    Ok(report)
}
