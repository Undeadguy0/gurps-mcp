//! import-character: файл gurps-sheet -> `character`, рёбра, журнал очков, пак `custom_<id>`.

use super::matcher::Matcher;
use super::{custom_pack, EDGES};
use crate::catalog::{export, import as cimport, items, modifiers, skills, spells, traits, Resolver};
use crate::db::Db;
use crate::ids;
use crate::report::{Dir, Report};
use crate::sheet::{Character, Equipment, LedgerKind, NegativeRounding};
use crate::sql::{Rec, Sv};
use serde_json::Value as J;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CharOpts {
    pub id: Option<String>,
    pub tl: String,
    pub spell_class: String,
    pub spell_system: String,
}

pub fn char_id(path: &Path, opt: Option<&str>) -> anyhow::Result<String> {
    let raw = match opt {
        Some(x) => x.to_string(),
        None => {
            let stem = path.file_name().and_then(|s| s.to_str()).unwrap_or("character");
            let stem = stem.split('.').next().unwrap_or(stem);
            ids::slug(stem)
        }
    };
    if raw.is_empty() || !raw.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        anyhow::bail!("id персонажа {raw:?}: нужен ^[a-z0-9_]+$ (задайте --id)");
    }
    Ok(raw)
}

fn j<T: serde::Serialize>(x: T) -> J {
    serde_json::to_value(x).unwrap()
}

pub struct Plan {
    pub id: String,
    pub sql: String,
    pub need: BTreeMap<Rec, String>,
}

fn sv_pair(k: &str, v: Sv) -> Sv {
    // явное удаление поля при MERGE
    let _ = k;
    if v == Sv::None {
        Sv::Raw("NONE".into())
    } else {
        v
    }
}

pub async fn plan(db: &Db, ch: &Character, id: &str, opts: &CharOpts, report: &mut Report) -> anyhow::Result<Plan> {
    let cref = Rec::code("character", id);
    let pack = custom_pack(id);
    let resolver = Resolver::load(db, &[]).await?;
    let colleges = export::college_names(db).await?;
    let mut m = Matcher {
        db,
        resolver,
        pack: pack.clone(),
        spell_class: opts.spell_class.clone(),
        colleges,
        customs: vec![],
        placed: BTreeMap::new(),
        report,
    };

    // custom-библиотека персонажа
    for d in &ch.custom.skills {
        let sid = ids::kebab_to_sid(&d.id)?;
        m.place(
            "skill",
            &sid,
            None,
            d,
            skills::to_db,
            |j, r, _| j_(skills::from_db(j, r)),
            &format!("custom skill {}", d.id),
        )
        .await?;
    }
    for d in &ch.custom.traits {
        let sid = ids::kebab_to_sid(&d.id)?;
        m.place(
            "trait",
            &sid,
            None,
            d,
            traits::to_db,
            |j, r, c| j_(traits::from_db(j, r, c)),
            &format!("custom trait {}", d.id),
        )
        .await?;
    }
    for d in &ch.custom.spells {
        let sid = ids::kebab_to_sid(&d.id)?;
        m.place(
            "spell",
            &sid,
            None,
            d,
            spells::to_db,
            |j, r, c| j_(spells::from_db(j, r, c)),
            &format!("custom spell {}", d.id),
        )
        .await?;
    }
    for d in &ch.custom.equipment {
        let sid = eq_sid(d.id.as_str(), &d.name)?;
        let s2 = sid.clone();
        m.place(
            "item",
            &sid,
            None,
            d,
            move |c, d| items::to_db(c, d, &s2, &[]),
            |j, r, _| j_(items::from_db(j, r).0),
            &format!("custom equipment {}", d.name),
        )
        .await?;
    }

    let mut edges = String::new();
    // черты
    for (i, t) in ch.traits.iter().enumerate() {
        let sid = ids::kebab_to_sid(&t.def.id)?;
        let what = format!("trait {}", t.def.id);
        let rec = m
            .place(
                "trait",
                &sid,
                t.source.as_deref(),
                &t.def,
                traits::to_db,
                |j, r, c| j_(traits::from_db(j, r, c)),
                &what,
            )
            .await?;
        let leveled = t.def.cost_per_level != 0;
        if !leveled && t.levels != 0 {
            m.report
                .gap(Dir::Import, "trait.levels у черты без уровней: отброшено", what.clone());
        }
        let mut mods = Vec::new();
        for md in &t.modifiers {
            let msid = ids::kebab_to_sid(&md.def.id)?;
            let mr = m
                .place(
                    "modifier",
                    &msid,
                    md.source.as_deref(),
                    &md.def,
                    modifiers::to_db,
                    |j, r, _| j_(modifiers::from_db(j, r)),
                    &format!("modifier {}", md.def.id),
                )
                .await?;
            if md.explanation.is_some() {
                m.report
                    .gap(Dir::Import, "trait.modifiers[].explanation: в схеме нет", what.clone());
            }
            mods.push(Sv::obj().with("modifier", Sv::Rec(mr)));
        }
        let c = Sv::obj()
            .with("level", if leveled { Sv::Int(t.levels as i64) } else { Sv::None })
            .with("modifiers", Sv::Arr(mods))
            .with("note", Sv::opt(t.explanation.clone().filter(|e| !e.trim().is_empty()), Sv::Str));
        relate(&mut edges, "has_trait", id, i, &cref, &rec, c);
    }
    // навыки
    for (i, s) in ch.skills.iter().enumerate() {
        let sid = ids::kebab_to_sid(&s.def.id)?;
        let what = format!("skill {}", s.def.id);
        let rec = m
            .place(
                "skill",
                &sid,
                s.source.as_deref(),
                &s.def,
                skills::to_db,
                |j, r, _| j_(skills::from_db(j, r)),
                &what,
            )
            .await?;
        let mut c = Sv::obj().with("points", Sv::Int(s.points as i64));
        if let Some(sp) = s.specialization.as_deref().filter(|x| !x.trim().is_empty()) {
            let ssid = skills::spec_sid(sp);
            if s.def.specializations.iter().any(|x| skills::spec_sid(x) == ssid) {
                c.set("spec", Sv::Str(ssid));
            } else {
                m.report.gap(
                    Dir::Import,
                    "skill.specialization вне списка specializations навыка: в схеме нельзя, отброшено",
                    format!("{what}: {sp}"),
                );
            }
        }
        relate(&mut edges, "has_skill", id, i, &cref, &rec, with_note(c, &s.explanation));
    }
    for (i, t) in ch.techniques.iter().enumerate() {
        let sid = ids::kebab_to_sid(&t.def.id)?;
        let what = format!("technique {}", t.def.id);
        let rec = m
            .place(
                "technique",
                &sid,
                t.source.as_deref(),
                &t.def,
                skills::tech_to_db,
                |j, r, _| j_(skills::tech_from_db(j, r)),
                &what,
            )
            .await?;
        relate(
            &mut edges,
            "has_technique",
            id,
            i,
            &cref,
            &rec,
            with_note(Sv::obj().with("points", Sv::Int(t.points as i64)), &t.explanation),
        );
    }
    for (i, s) in ch.spells.iter().enumerate() {
        let sid = ids::kebab_to_sid(&s.def.id)?;
        let what = format!("spell {}", s.def.id);
        let rec = m
            .place(
                "spell",
                &sid,
                s.source.as_deref(),
                &s.def,
                spells::to_db,
                |j, r, c| j_(spells::from_db(j, r, c)),
                &what,
            )
            .await?;
        relate(
            &mut edges,
            "has_spell",
            id,
            i,
            &cref,
            &rec,
            with_note(Sv::obj().with("points", Sv::Int(s.points as i64)), &s.explanation),
        );
    }
    // снаряжение (дерево)
    let mut items_sql = String::new();
    let mut stack: Vec<(Vec<i64>, &Equipment)> = ch.equipment.iter().enumerate().rev().map(|(i, e)| (vec![i as i64], e)).collect();
    while let Some((path, e)) = stack.pop() {
        let sid = eq_sid(&e.def.id, &e.def.name)?;
        if e.def.id.is_empty() {
            m.report.gap(
                Dir::Import,
                "equipment без id: sid выведен из имени, при экспорте id = sid",
                e.def.name.clone(),
            );
        }
        let what = format!("equipment {}", e.def.name);
        let s2 = sid.clone();
        let rec = m
            .place(
                "item",
                &sid,
                e.source.as_deref(),
                &e.def,
                move |c, d| items::to_db(c, d, &s2, &[]),
                |j, r, _| j_(items::from_db(j, r).0),
                &what,
            )
            .await?;
        let mut c = with_note(Sv::obj(), &e.explanation)
            .with("qty", Sv::Int(e.quantity.max(1) as i64))
            .with("equipped", Sv::Bool(e.carried));
        if e.quantity == 0 {
            m.report
                .gap(Dir::Import, "equipment.quantity = 0: в схеме qty >= 1, записано 1", what.clone());
        }
        if path.len() > 1 {
            c.set("contained_in", Sv::Rec(item_edge(id, &path[..path.len() - 1])));
        }
        let eid = item_edge(id, &path);
        let full = c
            .with("id", Sv::Rec(eid))
            .with("in", Sv::Rec(cref.clone()))
            .with("out", Sv::Rec(rec));
        let _ = writeln!(items_sql, "INSERT RELATION INTO owns_item {};", full.to_sql());
        for (k, sub) in e.contents.iter().enumerate().rev() {
            let mut p = path.clone();
            p.push(k as i64);
            stack.push((p, sub));
        }
    }
    if !ch.weapons.is_empty() {
        m.report.gap(
            Dir::Import,
            "weapons: снимок таблицы оружия не хранится (выводится из предметов)",
            format!("{} шт.", ch.weapons.len()),
        );
    }
    let customs = std::mem::take(&mut m.customs);
    let report = m.report;

    // персонаж
    let a = &ch.attributes;
    let vals = [
        a.st,
        a.dx,
        a.iq,
        a.ht,
        a.hp,
        a.will,
        a.per,
        a.fp,
        a.basic_speed_quarters,
        a.basic_move,
    ];
    let attrs: Vec<Sv> = super::ATTRS
        .iter()
        .zip(vals)
        .filter(|(_, v)| *v != 0)
        .map(|(c, v)| {
            Sv::obj()
                .with("attribute", Sv::Rec(Rec::code("ref_attribute", c)))
                .with("levels", Sv::Int(v as i64))
        })
        .collect();
    if ch.rules.negative_rounding != NegativeRounding::default() {
        report.gap(
            Dir::Import,
            "rules.negative_rounding: опция стола, а не факт персонажа; в схеме нет, расчёт по правилу Basic Set",
            id,
        );
    }
    report.gap(Dir::Import, "character.tl: в листе нет, взято --tl", format!("{id}: {}", opts.tl));
    let mut cc = Sv::obj()
        .with("name", Sv::str(&ch.name))
        .with("tl", Sv::Rec(Rec::code("ref_tl", &opts.tl)))
        .with("attributes", Sv::Arr(attrs));
    for (k, v) in [
        ("player", Sv::opt(Some(ch.player.clone()).filter(|p| !p.is_empty()), Sv::Str)),
        ("notes", Sv::opt(Some(ch.notes.clone()).filter(|p| !p.is_empty()), Sv::Str)),
        ("disadv_limit", Sv::opt(ch.rules.disadvantage_limit, |x| Sv::Int(x as i64))),
        ("current_hp", Sv::opt(ch.session.hp, |x| Sv::Int(x as i64))),
        ("current_fp", Sv::opt(ch.session.fp, |x| Sv::Int(x as i64))),
    ] {
        if let Sv::Obj(o) = &mut cc {
            o.insert(k.into(), sv_pair(k, v));
        }
    }

    let mut sql = String::new();
    for e in EDGES {
        let _ = writeln!(sql, "DELETE {e} WHERE in = {cref};");
    }
    let _ = writeln!(sql, "DELETE point_ledger WHERE character = {cref};");
    let _ = writeln!(sql, "UPSERT {cref} MERGE {};", cc.to_sql());
    for t in crate::catalog::TABLES {
        let _ = writeln!(sql, "DELETE {t} WHERE pack = pack:{pack};");
    }
    if customs.is_empty() {
        let _ = writeln!(sql, "DELETE pack:{pack};");
    } else {
        let mut pc = Sv::obj()
            .with("name", Sv::str(format!("Custom: {}", ch.name)))
            .with("abbrev", Sv::str(format!("C-{id}")))
            .with("kind", Sv::str("custom"))
            .with("owner", Sv::Rec(cref.clone()))
            .with("priority", Sv::Int(1000));
        if customs.iter().any(|e| e.rec.table == "spell") {
            pc.set("spell_system", Sv::Rec(Rec::code("ref_spell_system", &opts.spell_system)));
        }
        let _ = writeln!(sql, "UPSERT pack:{pack} MERGE {};", pc.to_sql());
        for t in crate::catalog::TABLES {
            for e in customs.iter().filter(|e| e.rec.table == t) {
                cimport::entity_sql(&mut sql, e);
            }
        }
        for e in &customs {
            cimport::edges_sql(&mut sql, e);
            cimport::translations_sql(&mut sql, e);
        }
        report.info(format!("пак pack:{pack}: {} записей", customs.len()));
    }
    sql.push_str(&edges);
    sql.push_str(&items_sql);
    ledger_sql(&mut sql, ch, id, &cref, report);

    let mut need = BTreeMap::new();
    cimport::ref_checks(&customs, &mut need);
    need.insert(Rec::code("ref_tl", &opts.tl), format!("{cref}.tl"));
    for c in super::ATTRS {
        need.insert(Rec::code("ref_attribute", c), format!("{cref}.attributes"));
    }
    Ok(Plan {
        id: id.to_string(),
        sql,
        need,
    })
}

fn j_<T: serde::Serialize>(x: T) -> J {
    j(x)
}

fn eq_sid(id: &str, name: &str) -> anyhow::Result<String> {
    if id.is_empty() {
        let s = ids::slug(name);
        if s.is_empty() {
            anyhow::bail!("equipment без id и имени");
        }
        Ok(s)
    } else {
        ids::kebab_to_sid(id)
    }
}

pub fn item_edge(id: &str, path: &[i64]) -> Rec {
    let mut parts = vec![crate::sql::KeyPart::S(id.into())];
    parts.extend(path.iter().map(|x| crate::sql::KeyPart::I(*x)));
    Rec {
        table: "owns_item".into(),
        key: crate::sql::Key::Arr(parts),
    }
}

fn relate(sql: &mut String, table: &str, id: &str, i: usize, cref: &Rec, out: &Rec, c: Sv) {
    let eid = Rec {
        table: table.into(),
        key: crate::sql::Key::Arr(vec![crate::sql::KeyPart::S(id.into()), crate::sql::KeyPart::I(i as i64)]),
    };
    let full = c
        .with("id", Sv::Rec(eid))
        .with("in", Sv::Rec(cref.clone()))
        .with("out", Sv::Rec(out.clone()));
    let _ = writeln!(sql, "INSERT RELATION INTO {table} {};", full.to_sql());
}

fn with_note(c: Sv, e: &Option<String>) -> Sv {
    match e.as_deref().filter(|x| !x.trim().is_empty()) {
        Some(x) => c.with("note", Sv::str(x)),
        None => c,
    }
}

fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    s.len() == 10 && b[4] == b'-' && b[7] == b'-' && s.chars().enumerate().all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

fn ledger_sql(sql: &mut String, ch: &Character, id: &str, cref: &Rec, report: &mut Report) {
    let rid = |n: i64| Rec {
        table: "point_ledger".into(),
        key: crate::sql::Key::Arr(vec![crate::sql::KeyPart::S(id.into()), crate::sql::KeyPart::I(n)]),
    };
    if ch.ledger.is_empty() {
        if ch.points_budget > 0 {
            let c = Sv::obj()
                .with("character", Sv::Rec(cref.clone()))
                .with("kind", Sv::str("award"))
                .with("delta", Sv::Int(ch.points_budget as i64))
                .with("note", Sv::str("starting"));
            let _ = writeln!(sql, "CREATE {} CONTENT {};", rid(0), c.to_sql());
        } else if ch.points_budget < 0 {
            let c = Sv::obj()
                .with("character", Sv::Rec(cref.clone()))
                .with("kind", Sv::str("adjust"))
                .with("delta", Sv::Int(ch.points_budget as i64));
            let _ = writeln!(sql, "CREATE {} CONTENT {};", rid(0), c.to_sql());
        }
        return;
    }
    report.gap(
        Dir::Import,
        "ledger.kind (вид начисления листа): в схеме только award/spend/refund/adjust",
        id,
    );
    for e in &ch.ledger {
        let kind = match (e.kind, e.amount) {
            (LedgerKind::Correction | LedgerKind::Other, _) => "adjust",
            (LedgerKind::ImpulseBuy | LedgerKind::BuyOff, a) if a < 0 => "spend",
            (_, a) if a > 0 => "award",
            _ => "adjust",
        };
        let mut c = Sv::obj()
            .with("character", Sv::Rec(cref.clone()))
            .with("kind", Sv::str(kind))
            .with("delta", Sv::Int(e.amount as i64))
            .with("note", Sv::opt(Some(e.note.clone()).filter(|n| !n.is_empty()), Sv::Str));
        if is_date(&e.date) {
            c.set("at", Sv::Raw(format!("d'{}T00:00:00Z'", e.date)));
        } else {
            report.gap(
                Dir::Import,
                "ledger.date: пусто/не YYYY-MM-DD, at = время импорта",
                format!("{id} #{}", e.id),
            );
        }
        if let Some(s) = &e.skill {
            match ids::kebab_to_sid(s) {
                Ok(sid) => {
                    c.set(
                        "target",
                        Sv::Raw(format!(
                            "(SELECT VALUE out FROM has_skill WHERE in = {cref} AND out.sid = {})[0]",
                            crate::sql::quote(&sid)
                        )),
                    );
                }
                Err(_) => report.gap(Dir::Import, "ledger.skill: неверный id", s.clone()),
            }
        }
        let _ = writeln!(sql, "CREATE {} CONTENT {};", rid(e.id as i64), c.to_sql());
    }
}

pub async fn run(db: &Db, path: &Path, opts: &CharOpts, dry_run: bool) -> anyhow::Result<Report> {
    let ch = crate::sheet::load_character(path)?;
    let id = char_id(path, opts.id.as_deref())?;
    let mut report = Report::default();
    let p = plan(db, &ch, &id, opts, &mut report).await?;
    cimport::check_refs(db, &p.need).await?;
    db.write(&p.sql, dry_run).await?;
    report.info(format!(
        "character:{} {}",
        p.id,
        if dry_run {
            "(dry-run, не записано)"
        } else {
            "записан"
        }
    ));
    Ok(report)
}
