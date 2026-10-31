//! Снаряжение и оружие: `EquipmentDef` + связанные `WeaponDef` -> `item` (attack_modes),
//! несвязанные `WeaponDef` (удар, пинок) -> `attack_mode` (natural = true).

use super::{damage, Ctx, Entity};
use crate::ids;
use crate::jsonx;
use crate::report::{Dir, Report};
use crate::sheet::{EquipmentDef, WeaponDef};
use crate::sql::Sv;
use serde_json::Value as J;

fn weapon_losses(ctx: &mut Ctx, w: &WeaponDef, e: &EquipmentDef) {
    let what = format!("weapon {}", w.id);
    if w.id != e.id {
        ctx.gap("weapon.id ≠ equipment.id: id оружия не сохраняется", what.clone());
    }
    if w.name != e.name {
        ctx.gap("weapon.name ≠ equipment.name: имя оружия не сохраняется", what.clone());
    }
    if w.page.is_some() && w.page != e.page {
        ctx.gap("weapon.page ≠ equipment.page: отброшено", what.clone());
    }
    if w.description.is_some() || ctx.locales.values().any(|l| l.contains_key(&format!("weapon-{}", w.id))) {
        ctx.gap(
            "weapon.description/перевод оружия, связанного со снаряжением: в схеме нет, отброшено",
            what,
        );
    }
}

/// Предмет; `sid` — уже вычисленный sid (у предметов из персонажа id может быть пустым).
pub fn to_db(ctx: &mut Ctx, e: &EquipmentDef, sid: &str, weapons: &[&WeaponDef]) -> anyhow::Result<Entity> {
    let rec = crate::sql::Rec::cat("item", &ctx.pack, sid);
    let what = format!("equipment {}", if e.id.is_empty() { &e.name } else { &e.id });
    let stats = e.stats.as_deref().map(str::trim).unwrap_or("");
    let mut category = if weapons.is_empty() { "gear" } else { "melee" };
    let mut c = Sv::obj();
    if let Some(db) = stats.strip_prefix("DB ").and_then(|x| x.trim().parse::<i64>().ok()) {
        category = "shield";
        let mut sh = Sv::obj().with("db", Sv::Int(db));
        // В листе навык блока не задан: щит блокируется навыком Shield (B358), если он известен.
        match ctx.find("skill", "shield") {
            Some(r) => {
                sh.set("block_skill", Sv::obj().with("skill", Sv::Rec(r)));
            }
            None => ctx.gap(
                "item.shield.block_skill: навык shield не найден, Block не вычисляется",
                what.clone(),
            ),
        }
        c.set("shield", sh);
    } else if let Some(dr) = stats.strip_prefix("DR ") {
        category = "armor";
        c.set("armor", Sv::obj().with("dr_raw", Sv::str(dr.trim())));
    } else if !stats.is_empty() {
        ctx.gap("equipment.stats: нераспознанная строка, отброшено", format!("{what}: {stats}"));
    }
    c.set("category", Sv::str(category));
    c.set("weight", super::num(e.weight));
    c.set("cost", super::num(e.cost));
    if e.container {
        c.set("container", Sv::obj());
    }
    super::set_page(ctx, &mut c, e.page.as_deref(), &what);
    let mut modes = Vec::new();
    for w in weapons {
        weapon_losses(ctx, w, e);
        for a in &w.attacks {
            let m = damage::attack_to_db(ctx, a, &w.skill, &format!("weapon {}", w.id)).with("name", Sv::str(&a.mode));
            modes.push(m);
        }
    }
    c.set("attack_modes", Sv::Arr(modes));
    let mut ent = Entity::new(rec, c);
    let key_id = if e.id.is_empty() { ids::sid_to_kebab(sid) } else { e.id.clone() };
    ent.tr = ctx.texts("equipment", &key_id, &e.name, e.description.as_ref());
    Ok(ent)
}

/// Оружие без снаряжения -> natural attack_mode (один режим).
pub fn natural_to_db(ctx: &mut Ctx, w: &WeaponDef) -> anyhow::Result<Entity> {
    let rec = ctx.rec("attack_mode", &w.id)?;
    let what = format!("weapon {}", w.id);
    let Some(a) = w.attacks.first() else {
        anyhow::bail!("{what}: нет ни одной атаки");
    };
    if w.attacks.len() > 1 {
        ctx.gap(
            "weapon без снаряжения с несколькими атаками: attack_mode хранит один режим, остальные отброшены",
            what.clone(),
        );
    }
    if a.mode != w.id {
        ctx.gap(
            "attack.mode у natural attack_mode: имени режима нет, при экспорте = id",
            what.clone(),
        );
    }
    let mut c = damage::attack_to_db(ctx, a, &w.skill, &what).with("natural", Sv::Bool(true));
    super::set_page(ctx, &mut c, w.page.as_deref(), &what);
    let mut e = Entity::new(rec, c);
    e.tr = ctx.texts("weapon", &w.id, &w.name, w.description.as_ref());
    Ok(e)
}

pub fn from_db(j: &J, report: &mut Report) -> (EquipmentDef, Option<WeaponDef>) {
    let id = super::kebab_sid(j);
    let what = format!("item {id}");
    let category = jsonx::s(j, "category").unwrap_or("gear");
    let mut stats = None;
    if let Some(sh) = j.get("shield").filter(|v| !v.is_null()) {
        stats = jsonx::i(sh, "db").map(|d| format!("DB {d}"));
        if ["parry", "dr", "hp"].iter().any(|k| sh.get(*k).is_some()) {
            report.gap(Dir::Export, "item.shield.parry/dr/hp: в листе нет", what.clone());
        }
    }
    if let Some(ar) = j.get("armor").filter(|v| !v.is_null()) {
        match jsonx::s(ar, "dr_raw") {
            Some(dr) => stats = Some(format!("DR {dr}")),
            None => report.gap(Dir::Export, "item.armor без dr_raw: DR не выводится", what.clone()),
        }
        if ar.get("locations").is_some() {
            report.gap(Dir::Export, "item.armor.locations: в листе нет", what.clone());
        }
    }
    if category == "ranged" {
        report.gap(Dir::Export, "item.category = ranged: в листе нет стрелкового оружия", what.clone());
    }
    for k in ["weight_raw", "tl_raw", "tl", "lc"] {
        if j.get(k).is_some_and(|v| !v.is_null()) {
            report.gap(Dir::Export, format!("item.{k}: в листе нет"), what.clone());
        }
    }
    if jsonx::i(j, "quantity").unwrap_or(1) != 1 {
        report.gap(Dir::Export, "item.quantity (размер упаковки): в листе нет", what.clone());
    }
    if !jsonx::arr(j, "features").is_empty() {
        report.gap(Dir::Export, "item.features: в листе нет", what.clone());
    }
    let container = j.get("container").is_some_and(|v| !v.is_null());
    if container && j.get("container").is_some_and(|c| c.as_object().is_some_and(|o| !o.is_empty())) {
        report.gap(Dir::Export, "item.container.capacity: в листе нет", what.clone());
    }
    let name = super::name_of(j, report, &what);
    let page = super::page_of(j);
    let modes = jsonx::arr(j, "attack_modes");
    let weapon = (!modes.is_empty()).then(|| {
        let skill = modes
            .iter()
            .find_map(|m| jsonx::sid_of(m, "skill"))
            .map(|s| ids::sid_to_kebab(&s))
            .unwrap_or_default();
        let attacks = modes
            .iter()
            .map(|m| {
                let mode = jsonx::s(m, "name").unwrap_or("").to_string();
                damage::attack_from_db(m, mode, &skill, report, &what)
            })
            .collect();
        WeaponDef {
            id: id.clone(),
            name: name.clone(),
            equipment: Some(id.clone()),
            skill,
            attacks,
            page: page.clone(),
            description: None,
        }
    });
    let eq = EquipmentDef {
        name,
        weight: jsonx::f(j, "weight").unwrap_or(0.0),
        cost: jsonx::f(j, "cost").unwrap_or(0.0),
        container,
        stats,
        page,
        description: super::desc_of(j),
        id,
    };
    (eq, weapon)
}

pub fn natural_from_db(j: &J, report: &mut Report) -> WeaponDef {
    let id = super::kebab_sid(j);
    let what = format!("attack_mode {id}");
    if jsonx::b(j, "natural") != Some(true) {
        report.gap(
            Dir::Export,
            "attack_mode с natural = false: выведено как оружие без снаряжения",
            what.clone(),
        );
    }
    if j.get("limb").is_some_and(|v| !v.is_null()) {
        report.gap(Dir::Export, "attack_mode.limb: в листе нет", what.clone());
    }
    let skill = jsonx::sid_of(j, "skill").map(|s| ids::sid_to_kebab(&s)).unwrap_or_default();
    let attack = damage::attack_from_db(j, id.clone(), &skill, report, &what);
    WeaponDef {
        name: super::name_of(j, report, &what),
        equipment: None,
        skill,
        attacks: vec![attack],
        page: super::page_of(j),
        description: super::desc_of(j),
        id,
    }
}
