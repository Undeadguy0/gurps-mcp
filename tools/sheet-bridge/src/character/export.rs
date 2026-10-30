//! export-character: `character` + рёбра + пак `custom_<id>` -> файл gurps-sheet.

use super::{custom_pack, ATTRS};
use crate::catalog::{export as cexport, items, modifiers, skills, spells, traits, Resolver};
use crate::db::Db;
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::*;
use crate::sql::Rec;
use serde_json::Value as J;
use std::collections::BTreeMap;
use std::path::Path;

struct Ex<'a> {
    db: &'a Db,
    report: &'a mut Report,
    resolver: Resolver,
    pack: String,
    colleges: BTreeMap<String, String>,
    cache: BTreeMap<Rec, J>,
}

impl Ex<'_> {
    async fn row(&mut self, rec: &Rec) -> anyhow::Result<J> {
        if let Some(j) = self.cache.get(rec) {
            return Ok(j.clone());
        }
        let rows = cexport::fetch(self.db, &rec.table, &format!("id = {rec}")).await?;
        let j = rows.into_iter().next().ok_or_else(|| anyhow::anyhow!("{rec} не найден"))?;
        self.cache.insert(rec.clone(), j.clone());
        Ok(j)
    }

    /// source: пак-источник; для записей custom-пака — пак с тем же sid (изменённый снимок) или None.
    fn source(&self, rec: &Rec) -> Option<String> {
        let (p, s) = rec.pack_sid()?;
        if p != self.pack {
            return Some(ids::sid_to_kebab(p));
        }
        self.resolver
            .find(&rec.table, s, "")
            .and_then(|r| r.pack_sid().map(|(p, _)| ids::sid_to_kebab(p)))
    }

    /// Запись из библиотечного (не custom) пака: описание лист берёт из локалей, в снимок не пишем.
    fn lib(&self, rec: &Rec) -> bool {
        rec.pack_sid().is_some_and(|(p, _)| p != self.pack)
    }

    async fn edges(&mut self, table: &str, cref: &Rec) -> anyhow::Result<Vec<J>> {
        let v = self
            .db
            .one(&format!("SELECT * FROM {table} WHERE in = {cref} ORDER BY id;"))
            .await?;
        Ok(v.as_array().cloned().unwrap_or_default())
    }
}

fn out_rec(e: &J) -> anyhow::Result<Rec> {
    jsonx::rec(e, "out").ok_or_else(|| anyhow::anyhow!("ребро без out: {e}"))
}

pub async fn build(db: &Db, id: &str, report: &mut Report) -> anyhow::Result<Character> {
    let cref = Rec::code("character", id);
    let c = db.one(&format!("SELECT * FROM ONLY {cref};")).await?;
    if c.is_null() {
        anyhow::bail!("{cref} не найден");
    }
    let pack = custom_pack(id);
    let resolver = Resolver::load(db, &[]).await?;
    let colleges = cexport::college_names(db).await?;
    let mut ex = Ex {
        db,
        report,
        resolver,
        pack: pack.clone(),
        colleges,
        cache: BTreeMap::new(),
    };
    let mut ch = Character {
        name: jsonx::s(&c, "name").unwrap_or("").into(),
        player: jsonx::s(&c, "player").unwrap_or("").into(),
        notes: jsonx::s(&c, "notes").unwrap_or("").into(),
        ..Character::default()
    };
    for (k, t) in [
        ("campaign", "character.campaign"),
        ("mana_level", "character.mana_level"),
        ("quirk_limit", "character.quirk_limit"),
    ] {
        if c.get(k).is_some_and(|v| !v.is_null()) {
            ex.report.gap(Dir::Export, format!("{t}: в листе нет"), id);
        }
    }
    ex.report.gap(Dir::Export, "character.tl/created_at: в листе нет", id);
    ch.rules.disadvantage_limit = jsonx::i(&c, "disadv_limit").map(|x| x as i32);
    // атрибуты
    let mut vals: BTreeMap<String, i32> = BTreeMap::new();
    for a in jsonx::arr(&c, "attributes") {
        if let Some(code) = jsonx::code(a, "attribute") {
            let lv = jsonx::f(a, "levels").unwrap_or(0.0);
            if lv.fract() != 0.0 {
                ex.report
                    .gap(Dir::Export, "attributes.levels дробное: округлено", format!("{id}: {code}"));
            }
            vals.insert(code, lv.round() as i32);
        }
    }
    for k in vals.keys().filter(|k| !ATTRS.contains(&k.as_str())) {
        ex.report.gap(Dir::Export, "атрибут вне набора листа", format!("{id}: {k}"));
    }
    let g = |k: &str| vals.get(k).copied().unwrap_or(0);
    ch.attributes = Attributes {
        st: g("ST"),
        dx: g("DX"),
        iq: g("IQ"),
        ht: g("HT"),
        hp: g("HP"),
        will: g("Will"),
        per: g("Per"),
        fp: g("FP"),
        basic_speed_quarters: g("Speed"),
        basic_move: g("Move"),
    };

    // черты
    for e in ex.edges("has_trait", &cref).await? {
        let rec = out_rec(&e)?;
        let j = ex.row(&rec).await?;
        let mut def = traits::from_db(&j, ex.report);
        if ex.lib(&rec) {
            def.description = None;
        }
        let mut mods = Vec::new();
        for m in jsonx::arr(&e, "modifiers") {
            let Some(mr) = jsonx::rec(m, "modifier") else { continue };
            if m.get("levels").is_some_and(|v| !v.is_null()) {
                ex.report
                    .gap(Dir::Export, "has_trait.modifiers[].levels: в листе нет", def.id.clone());
            }
            let mj = ex.row(&mr).await?;
            let mut md = modifiers::from_db(&mj, ex.report);
            if ex.lib(&mr) {
                md.description = None;
            }
            mods.push(TraitModifier {
                def: md,
                source: ex.source(&mr),
                explanation: None,
            });
        }
        ch.traits.push(Trait {
            levels: jsonx::i(&e, "level").unwrap_or(0) as u32,
            source: ex.source(&rec),
            explanation: jsonx::s(&e, "note").map(str::to_string),
            modifiers: mods,
            def,
        });
    }
    for e in ex.edges("has_skill", &cref).await? {
        let rec = out_rec(&e)?;
        let j = ex.row(&rec).await?;
        let mut def = skills::from_db(&j, ex.report);
        if ex.lib(&rec) {
            def.description = None;
        }
        let specialization = jsonx::s(&e, "spec").map(|sp| {
            jsonx::arr(&j, "specializations")
                .iter()
                .position(|x| x.as_str() == Some(sp))
                .and_then(|i| def.specializations.get(i).cloned())
                .unwrap_or_else(|| sp.to_string())
        });
        if e.get("tl").is_some_and(|v| !v.is_null()) {
            ex.report.gap(Dir::Export, "has_skill.tl: в листе нет", def.id.clone());
        }
        ch.skills.push(Skill {
            points: jsonx::i(&e, "points").unwrap_or(0) as i32,
            specialization,
            source: ex.source(&rec),
            explanation: None,
            def,
        });
    }
    for e in ex.edges("has_technique", &cref).await? {
        let rec = out_rec(&e)?;
        let j = ex.row(&rec).await?;
        let mut def = skills::tech_from_db(&j, ex.report);
        if ex.lib(&rec) {
            def.description = None;
        }
        ch.techniques.push(Technique {
            def,
            points: jsonx::i(&e, "points").unwrap_or(0) as i32,
            source: ex.source(&rec),
            explanation: None,
        });
    }
    for e in ex.edges("has_spell", &cref).await? {
        let rec = out_rec(&e)?;
        let j = ex.row(&rec).await?;
        let mut def = spells::from_db(&j, ex.report, &ex.colleges);
        if ex.lib(&rec) {
            def.description = None;
        }
        ch.spells.push(Spell {
            def,
            points: jsonx::i(&e, "points").unwrap_or(0) as i32,
            source: ex.source(&rec),
            explanation: None,
        });
    }
    // снаряжение: дерево по contained_in
    let owned = ex.edges("owns_item", &cref).await?;
    let mut nodes: Vec<(String, Option<String>, Equipment)> = Vec::new();
    for e in &owned {
        let rec = out_rec(e)?;
        let j = ex.row(&rec).await?;
        let (mut def, _) = items::from_db(&j, ex.report);
        if ex.lib(&rec) {
            def.description = None;
        }
        if e.get("location").is_some_and(|v| !v.is_null()) {
            ex.report.gap(Dir::Export, "owns_item.location: в листе нет", def.id.clone());
        }
        let eq = Equipment {
            def,
            quantity: jsonx::i(e, "qty").unwrap_or(1) as u32,
            carried: jsonx::b(e, "equipped").unwrap_or(false),
            contents: vec![],
            source: ex.source(&rec),
            explanation: None,
        };
        nodes.push((
            jsonx::s(e, "id").unwrap_or("").to_string(),
            jsonx::s(e, "contained_in").map(str::to_string),
            eq,
        ));
    }
    ch.equipment = tree(nodes);
    if !owned.is_empty() {
        ex.report
            .gap(Dir::Export, "weapons: снимок не выводится (лист строит таблицу оружия сам)", id);
    }
    // custom-библиотека: записи пака персонажа, которых нет в других паках
    let rows = cexport::fetch_pack(db, &pack).await?;
    let mut custom = CustomContent::default();
    let get = |t: &str| rows.rows.get(t).cloned().unwrap_or_default();
    let fresh = |ex: &Ex, j: &J, t: &str| jsonx::s(j, "sid").is_some_and(|s| ex.resolver.find(t, s, "").is_none());
    for j in get("trait") {
        if fresh(&ex, &j, "trait") {
            custom.traits.push(traits::from_db(&j, ex.report));
        }
    }
    for j in get("skill") {
        if fresh(&ex, &j, "skill") {
            custom.skills.push(skills::from_db(&j, ex.report));
        }
    }
    for j in get("spell") {
        if fresh(&ex, &j, "spell") {
            custom.spells.push(spells::from_db(&j, ex.report, &ex.colleges));
        }
    }
    for j in get("item") {
        if fresh(&ex, &j, "item") {
            custom.equipment.push(items::from_db(&j, ex.report).0);
        }
    }
    ch.custom = custom;
    ledger(db, &mut ch, &cref, ex.report).await?;
    Ok(ch)
}

fn tree(nodes: Vec<(String, Option<String>, Equipment)>) -> Vec<Equipment> {
    let mut children: BTreeMap<Option<String>, Vec<(String, Equipment)>> = BTreeMap::new();
    for (id, parent, e) in nodes {
        children.entry(parent).or_default().push((id, e));
    }
    fn take(key: Option<String>, ch: &mut BTreeMap<Option<String>, Vec<(String, Equipment)>>) -> Vec<Equipment> {
        let list = ch.remove(&key).unwrap_or_default();
        list.into_iter()
            .map(|(id, mut e)| {
                e.contents = take(Some(id), ch);
                e
            })
            .collect()
    }
    take(None, &mut children)
}

async fn ledger(db: &Db, ch: &mut Character, cref: &Rec, report: &mut Report) -> anyhow::Result<()> {
    let v = db
        .one(&format!("SELECT * FROM point_ledger WHERE character = {cref} ORDER BY id;"))
        .await?;
    let rows = v.as_array().cloned().unwrap_or_default();
    if rows.is_empty() {
        ch.points_budget = 0;
        return Ok(());
    }
    if rows.len() == 1 && jsonx::s(&rows[0], "kind") == Some("award") {
        ch.points_budget = jsonx::i(&rows[0], "delta").unwrap_or(0) as i32;
        return Ok(());
    }
    report.gap(
        Dir::Export,
        "ledger.kind: award/spend/adjust -> starting/session-award/impulse-buy/correction (приближённо)",
        cref.to_string(),
    );
    let mut sum = 0;
    for (n, r) in rows.iter().enumerate() {
        let amount = jsonx::i(r, "delta").unwrap_or(0) as i32;
        sum += amount;
        let kind = match jsonx::s(r, "kind") {
            Some("award") if n == 0 => LedgerKind::Starting,
            Some("award") => LedgerKind::SessionAward,
            Some("spend") => LedgerKind::ImpulseBuy,
            _ => LedgerKind::Correction,
        };
        let id = jsonx::rec(r, "id")
            .and_then(|x| match x.key {
                crate::sql::Key::Arr(v) => v
                    .last()
                    .and_then(|p| if let crate::sql::KeyPart::I(i) = p { Some(*i as u32) } else { None }),
                _ => None,
            })
            .unwrap_or(n as u32);
        let skill = jsonx::sid_of(r, "target")
            .filter(|_| jsonx::rec(r, "target").is_some_and(|t| t.table == "skill"))
            .map(|s| ids::sid_to_kebab(&s));
        if r.get("session").is_some_and(|v| !v.is_null()) {
            report.gap(Dir::Export, "ledger.session: в листе нет", cref.to_string());
        }
        ch.ledger.push(LedgerEntry {
            id,
            date: jsonx::s(r, "at").map(|a| a.chars().take(10).collect()).unwrap_or_default(),
            amount,
            kind,
            note: jsonx::s(r, "note").unwrap_or("").to_string(),
            skill,
        });
    }
    ch.points_budget = sum;
    Ok(())
}

pub async fn run(db: &Db, id: &str, out: &Path) -> anyhow::Result<Report> {
    let mut report = Report::default();
    let ch = build(db, id, &mut report).await?;
    if let Some(p) = out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(p)?;
    }
    std::fs::write(out, character_to_string(&ch)?)?;
    report.info(format!("character:{id} -> {}", out.display()));
    Ok(report)
}
