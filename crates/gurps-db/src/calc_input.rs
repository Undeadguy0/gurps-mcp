//! Загрузка входных данных расчёта из БД одним запросом (ссылки -> sid/code).
use crate::db::Db;
use crate::error::{DbError, DbResult};
use gurps_calc::model::Input;

/// Режим атаки (объект `$m`) -> форма [`gurps_calc::model::AttackModeDef`].
fn attack(m: &str, sid: &str) -> String {
    format!(
        "{{ sid: {sid}, name: {m}.name, natural: {m}.natural ?? false, skill: {m}.skill.sid,
           damage: {{ base: {m}.damage.base, dice: {m}.damage.dice, mod: {m}.damage.mod ?? 0,
                     type: IF {m}.damage.type {{ record::id({m}.damage.type) }} ELSE {{ NONE }} }},
           reach: {m}.reach ?? [], parry: {m}.parry.value,
           defaults: ({m}.defaults ?? []).map(|$d| {{
               attribute: IF $d.attribute {{ record::id($d.attribute) }} ELSE {{ NONE }},
               skill: $d.skill.sid, mod: $d.mod }}),
           skill_modifier: {m}.skill_modifier, skill_damage_bonus: {m}.skill_damage_bonus,
           technique: {m}.technique.sid }}"
    )
}

/// Справочники расчёта (общие для всех персонажей).
pub const REFS: &str = "{
    attributes: (SELECT code, kind, base_value, base_from.map(|$a| record::id($a)) AS base_from,
                        base_divisor, cost_per_level, step FROM ref_attribute),
    difficulties: (SELECT code, base_mod, technique_costs FROM ref_difficulty),
    skill_costs: (SELECT points, level_offset FROM ref_skill_cost),
    st_damage: (SELECT st, thr, sw FROM ref_st_damage),
    rules: (SELECT code, value, text FROM ref_rule),
    encumbrance: (SELECT code, level, max_load_bl, move_mult, dodge_mod FROM ref_encumbrance ORDER BY level)
}";

/// Откуда берутся владения персонажа: рёбра БД или документ `$ch` (лист в памяти).
struct Src {
    prelude: &'static str,
    attributes: &'static str,
    current: &'static str,
    carried: &'static str,
    traits: &'static str,
    skills: &'static str,
    techniques: &'static str,
    spells: &'static str,
    items: &'static str,
    ledger: &'static str,
    overrides: &'static str,
    /// Позиция записи в документе (`i`) — для вставки встроенных определений.
    idx: &'static str,
    /// Доп. поля результата (только документ).
    extra: &'static str,
    /// Предмет при себе с учётом контейнеров.
    equipped: &'static str,
}

const DB_SRC: Src = Src {
    prelude: "LET $c = type::record('character', $id);",
    attributes: "$c.attributes.map(|$a| { code: record::id($a.attribute), levels: $a.levels })",
    current: "current_hp: $c.current_hp, current_fp: $c.current_fp,",
    carried: "math::sum((SELECT VALUE qty * (out.weight ?? 0) FROM owns_item WHERE in = $c AND equipped
                AND (contained_in IS NONE OR contained_in.equipped = true)
                AND (contained_in.contained_in IS NONE OR contained_in.contained_in.equipped = true)))",
    traits: "has_trait WHERE in = $c",
    skills: "has_skill WHERE in = $c",
    techniques: "has_technique WHERE in = $c",
    spells: "has_spell WHERE in = $c",
    items: "owns_item WHERE in = $c",
    ledger: "(SELECT kind, delta FROM point_ledger WHERE character = $c)",
    overrides: "[]",
    idx: "",
    extra: "",
    equipped: "(equipped AND (contained_in IS NONE OR contained_in.equipped = true)
        AND (contained_in.contained_in IS NONE OR contained_in.contained_in.equipped = true)) AS equipped",
};

/// Документ: `{ attributes: [{code, levels}], current_hp?, current_fp?,
///   traits: [{pack, sid, level?, modifiers: [{pack, sid, levels?}]}], skills: [{pack, sid, points, spec?}],
///   techniques/spells: [{pack, sid, points}], items: [{pack, sid, qty, equipped}] (плоско; equipped = при себе),
///   ledger: [{kind, delta}], rule_overrides: [{code, value?, text?}] }`.
///
/// Вместо `pack`/`sid` запись (и модификатор черты) может нести `inline` — встроенное определение
/// (своё содержимое листа, в каталоге его нет) в форме входа расчёта: черта — поля [`OwnedTrait`]
/// без `level`/`modifiers`, навык — [`OwnedSkill`] без `points`/`spec`, техника/заклинание — без
/// `points`, предмет — [`OwnedItem`] без `equipped` плюс `weight`; модификатор — [`ModVal`].
/// Ссылки внутри (навык, атрибут, школа) — ключи (sid/code).
///
/// [`OwnedTrait`]: gurps_calc::model::OwnedTrait
/// [`OwnedSkill`]: gurps_calc::model::OwnedSkill
/// [`OwnedItem`]: gurps_calc::model::OwnedItem
/// [`ModVal`]: gurps_calc::model::ModVal
const DOC_SRC: Src = Src {
    prelude: "LET $c = NONE;
        LET $tr = (SELECT i, level, type::record('trait', [pack, sid]) AS out,
            (modifiers ?? []).filter(|$m| $m.inline IS NONE)
                .map(|$m| { modifier: type::record('modifier', [$m.pack, $m.sid]), levels: $m.levels }) AS modifiers
            FROM ($ch.traits ?? []) WHERE inline IS NONE);
        LET $md = ($ch.traits ?? []).map(|$t| ($t.modifiers ?? []).filter(|$m| $m.inline IS NONE).map(|$m| {
            type: type::record('modifier', [$m.pack, $m.sid]).type,
            value_kind: type::record('modifier', [$m.pack, $m.sid]).value.kind,
            amount: type::record('modifier', [$m.pack, $m.sid]).value.amount,
            levels: $m.levels ?? type::record('modifier', [$m.pack, $m.sid]).levels }));
        LET $sk = (SELECT i, points, spec, type::record('skill', [pack, sid]) AS out FROM ($ch.skills ?? []) WHERE inline IS NONE);
        LET $te = (SELECT i, points, type::record('technique', [pack, sid]) AS out FROM ($ch.techniques ?? []) WHERE inline IS NONE);
        LET $sp = (SELECT i, points, type::record('spell', [pack, sid]) AS out FROM ($ch.spells ?? []) WHERE inline IS NONE);
        LET $it = (SELECT i, qty ?? 1 AS qty, equipped ?? false AS equipped, type::record('item', [pack, sid]) AS out
            FROM ($ch.items ?? []) WHERE inline IS NONE);",
    attributes: "($ch.attributes ?? [])",
    current: "current_hp: $ch.current_hp, current_fp: $ch.current_fp,",
    carried: "math::sum((SELECT VALUE qty * (out.weight ?? 0) FROM $it WHERE equipped))",
    traits: "$tr",
    skills: "$sk",
    techniques: "$te",
    spells: "$sp",
    items: "$it",
    ledger: "($ch.ledger ?? [])",
    overrides: "($ch.rule_overrides ?? [])",
    idx: "i,",
    extra: ", doc_modifiers: $md",
    equipped: "equipped",
};

fn build(s: &Src) -> String {
    let natural = attack("$m", "$m.sid");
    let item = attack("$m", "string::concat('item/', $m.name ?? '')");
    let Src {
        prelude,
        attributes,
        current,
        carried,
        traits,
        skills,
        techniques,
        spells,
        items,
        ledger,
        overrides,
        idx,
        extra,
        equipped,
    } = s;
    let guard = if std::ptr::eq(s, &DB_SRC) {
        "!record::exists($c)"
    } else {
        "false"
    };
    format!(
        "{prelude}
        RETURN IF {guard} {{ NONE }} ELSE {{ {{
            refs: {REFS},
            attributes: {attributes},
            {current}
            carried_weight: {carried},
            traits: (SELECT {idx} out.sid AS sid, out.kind AS kind, level, out.cost AS cost,
                        modifiers.map(|$m| {{ type: $m.modifier.type, value_kind: $m.modifier.value.kind,
                            amount: $m.modifier.value.amount, levels: $m.levels ?? $m.modifier.levels }}) AS modifiers,
                        out.features AS features, out.prereqs AS prereqs
                     FROM {traits}),
            skills: (SELECT {idx} out.sid AS sid, record::id(out.attribute) AS attribute,
                        record::id(out.difficulty) AS difficulty, points, spec,
                        out.features AS features, out.prereqs AS prereqs
                     FROM {skills}),
            techniques: (SELECT {idx} out.sid AS sid, out.parent_skill.sid AS parent_skill,
                        record::id(out.difficulty) AS difficulty, out.default_mod AS default_mod,
                        out.max_mod AS max_mod, points, out.features AS features, out.prereqs AS prereqs
                     FROM {techniques}),
            spells: (SELECT {idx} out.sid AS sid, out.colleges.map(|$x| record::id($x)) AS colleges,
                        record::id(out.difficulty) AS difficulty, points, out.prereqs AS prereqs
                     FROM {spells}),
            items: (SELECT {idx} out.sid AS sid, {equipped}, qty, out.weight AS weight, out.cost AS cost,
                        (out.armor.locations ?? []).map(|$l| {{ loc: record::id($l.loc), dr: $l.dr }}) AS armor,
                        out.features AS features,
                        IF out.shield {{ {{ db: out.shield.db, block_skill: out.shield.block_skill.skill.sid,
                            block_spec: out.shield.block_skill.spec }} }} ELSE {{ NONE }} AS shield,
                        (out.attack_modes ?? []).map(|$m| {item}) AS attack_modes
                     FROM {items}),
            natural_attacks: array::distinct((SELECT VALUE sid FROM attack_mode WHERE natural = true))
                .map(|$s| fn::effective('attack_mode', $s))
                .filter(|$m| $m != NONE AND $m.natural = true)
                .map(|$m| {natural}),
            defaults: array::concat(
                (SELECT skill.sid AS skill, record::id(attribute) AS attribute, mod FROM attr_default),
                (SELECT in.sid AS skill, out.sid AS target, mod FROM defaults_to)),
            catalog_skills: (SELECT sid, record::id(attribute) AS attribute FROM skill),
            ledger: {ledger},
            rule_overrides: {overrides}{extra}
        }} }};"
    )
}

/// Запрос для персонажа `character:$id` в БД.
pub fn query() -> String {
    build(&DB_SRC)
}

/// Запрос для документа персонажа `$ch` (каталог и справочники — из БД).
pub fn doc_query() -> String {
    build(&DOC_SRC)
}

/// Входные данные расчёта для персонажа `character:<id>`.
pub async fn load(db: &Db, id: &str) -> DbResult<Input> {
    let v = db.one(&query(), vec![("id", id.to_string())]).await?;
    if v.is_null() {
        return Err(DbError::NotFound(format!("character:{id}")));
    }
    decode(v)
}

/// Разделы документа с записями каталога или встроенными определениями.
const SECTIONS: [&str; 5] = ["traits", "skills", "techniques", "spells", "items"];
/// Поля экземпляра (не определения), переносимые из записи документа во встроенную запись.
const INSTANCE: [&str; 5] = ["level", "points", "spec", "qty", "equipped"];

/// Входные данные расчёта для документа персонажа (формат — см. [`DOC_SRC`]): записи каталога по
/// `(pack, sid)` — из БД, встроенные (`inline`) — как есть, в исходном порядке.
pub async fn load_doc(db: &Db, doc: &serde_json::Value) -> DbResult<Input> {
    let mut d = doc.clone();
    for s in SECTIONS {
        if let Some(a) = d.get_mut(s).and_then(|v| v.as_array_mut()) {
            for (i, e) in a.iter_mut().enumerate() {
                if let Some(o) = e.as_object_mut() {
                    o.insert("i".into(), i.into());
                }
            }
        }
    }
    let mut v = db.one_json(&doc_query(), "ch", d).await?;
    merge_inline(&mut v, doc)?;
    decode(v)
}

fn inline_mods(e: &serde_json::Value) -> impl Iterator<Item = serde_json::Value> + '_ {
    e.get("modifiers")
        .and_then(|m| m.as_array())
        .into_iter()
        .flatten()
        .filter_map(|m| m.get("inline").cloned())
}

/// Вставляет встроенные определения документа в результат запроса (по позиции `i`).
fn merge_inline(v: &mut serde_json::Value, doc: &serde_json::Value) -> DbResult<()> {
    use serde_json::Value as J;
    let Some(root) = v.as_object_mut() else {
        return Err(DbError::Decode("данные расчёта: не объект".into()));
    };
    let mods = root.remove("doc_modifiers").unwrap_or(J::Null);
    let mut extra_weight = 0.0;
    for s in SECTIONS {
        let entries = doc
            .get(s)
            .and_then(J::as_array)
            .cloned()
            .unwrap_or_default();
        let rows = match root.remove(s) {
            Some(J::Array(a)) => a,
            _ => Vec::new(),
        };
        let mut by_i = std::collections::BTreeMap::new();
        for mut r in rows {
            let i = r.get("i").and_then(J::as_u64).unwrap_or(u64::MAX) as usize;
            if let Some(o) = r.as_object_mut() {
                o.remove("i");
            }
            by_i.insert(i, r);
        }
        let mut out = Vec::with_capacity(entries.len());
        for (i, e) in entries.iter().enumerate() {
            let mut row = match e.get("inline") {
                Some(J::Object(def)) => {
                    let mut r = def.clone();
                    for k in INSTANCE {
                        if let Some(x) = e.get(k) {
                            r.insert(k.into(), x.clone());
                        }
                    }
                    if s == "items" && r.get("equipped").and_then(J::as_bool).unwrap_or(false) {
                        let qty = r.get("qty").and_then(J::as_f64).unwrap_or(1.0);
                        extra_weight += qty * r.get("weight").and_then(J::as_f64).unwrap_or(0.0);
                    }
                    J::Object(r)
                }
                Some(_) => {
                    return Err(DbError::Decode(format!(
                        "{s}[{i}].inline: ожидается объект"
                    )));
                }
                None => match by_i.remove(&i) {
                    Some(r) => r,
                    None => continue,
                },
            };
            if s == "traits" {
                let mut m: Vec<J> = mods
                    .get(i)
                    .and_then(J::as_array)
                    .cloned()
                    .unwrap_or_default();
                m.extend(inline_mods(e));
                row["modifiers"] = J::Array(m);
            }
            out.push(row);
        }
        root.insert(s.into(), J::Array(out));
    }
    if extra_weight != 0.0 {
        let w = root
            .get("carried_weight")
            .and_then(J::as_f64)
            .unwrap_or(0.0);
        root.insert("carried_weight".into(), (w + extra_weight).into());
    }
    Ok(())
}
fn decode(v: serde_json::Value) -> DbResult<Input> {
    serde_json::from_value(v).map_err(|e| DbError::Decode(format!("данные расчёта: {e}")))
}

/// Документ персонажа (формат [`load_doc`]) из `character:<id>` в БД: рёбра -> `(pack, sid)`,
/// снаряжение плоско, `equipped` учитывает контейнеры (до двух уровней).
pub async fn doc_of(db: &Db, id: &str) -> DbResult<serde_json::Value> {
    let q = "LET $c = type::record('character', $id);
        RETURN IF !record::exists($c) { NONE } ELSE { {
            attributes: $c.attributes.map(|$a| { code: record::id($a.attribute), levels: $a.levels }),
            current_hp: $c.current_hp, current_fp: $c.current_fp,
            traits: (SELECT record::id(out)[0] AS pack, out.sid AS sid, level,
                modifiers.map(|$m| { pack: record::id($m.modifier)[0], sid: $m.modifier.sid, levels: $m.levels }) AS modifiers
                FROM has_trait WHERE in = $c),
            skills: (SELECT record::id(out)[0] AS pack, out.sid AS sid, points, spec FROM has_skill WHERE in = $c),
            techniques: (SELECT record::id(out)[0] AS pack, out.sid AS sid, points FROM has_technique WHERE in = $c),
            spells: (SELECT record::id(out)[0] AS pack, out.sid AS sid, points FROM has_spell WHERE in = $c),
            items: (SELECT record::id(out)[0] AS pack, out.sid AS sid, qty,
                (equipped AND (contained_in IS NONE OR contained_in.equipped = true)
                    AND (contained_in.contained_in IS NONE OR contained_in.contained_in.equipped = true)) AS equipped
                FROM owns_item WHERE in = $c),
            ledger: (SELECT kind, delta FROM point_ledger WHERE character = $c),
            rule_overrides: []
        } };";
    let v = db.one(q, vec![("id", id.to_string())]).await?;
    if v.is_null() {
        return Err(DbError::NotFound(format!("character:{id}")));
    }
    Ok(v)
}
