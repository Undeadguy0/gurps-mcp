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
};

/// Документ: `{ attributes: [{code, levels}], current_hp?, current_fp?,
///   traits: [{pack, sid, level?, modifiers: [{pack, sid, levels?}]}], skills: [{pack, sid, points, spec?}],
///   techniques/spells: [{pack, sid, points}], items: [{pack, sid, qty, equipped}] (плоско; equipped = при себе),
///   ledger: [{kind, delta}], rule_overrides: [{code, value?, text?}] }`.
const DOC_SRC: Src = Src {
    prelude: "LET $c = NONE;
        LET $tr = (SELECT level, type::record('trait', [pack, sid]) AS out,
            (modifiers ?? []).map(|$m| { modifier: type::record('modifier', [$m.pack, $m.sid]), levels: $m.levels }) AS modifiers
            FROM ($ch.traits ?? []));
        LET $sk = (SELECT points, spec, type::record('skill', [pack, sid]) AS out FROM ($ch.skills ?? []));
        LET $te = (SELECT points, type::record('technique', [pack, sid]) AS out FROM ($ch.techniques ?? []));
        LET $sp = (SELECT points, type::record('spell', [pack, sid]) AS out FROM ($ch.spells ?? []));
        LET $it = (SELECT qty ?? 1 AS qty, equipped ?? false AS equipped, type::record('item', [pack, sid]) AS out
            FROM ($ch.items ?? []));",
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
            traits: (SELECT out.sid AS sid, level, out.cost AS cost,
                        modifiers.map(|$m| {{ type: $m.modifier.type, value_kind: $m.modifier.value.kind,
                            amount: $m.modifier.value.amount, levels: $m.levels ?? $m.modifier.levels }}) AS modifiers,
                        out.features AS features, out.prereqs AS prereqs
                     FROM {traits}),
            skills: (SELECT out.sid AS sid, record::id(out.attribute) AS attribute,
                        record::id(out.difficulty) AS difficulty, points, spec,
                        out.features AS features, out.prereqs AS prereqs
                     FROM {skills}),
            techniques: (SELECT out.sid AS sid, out.parent_skill.sid AS parent_skill,
                        record::id(out.difficulty) AS difficulty, out.default_mod AS default_mod,
                        out.max_mod AS max_mod, points, out.features AS features, out.prereqs AS prereqs
                     FROM {techniques}),
            spells: (SELECT out.sid AS sid, out.colleges.map(|$x| record::id($x)) AS colleges,
                        record::id(out.difficulty) AS difficulty, points, out.prereqs AS prereqs
                     FROM {spells}),
            items: (SELECT out.sid AS sid, equipped, out.features AS features,
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
            rule_overrides: {overrides}
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

/// Входные данные расчёта для документа персонажа (формат — см. [`DOC_SRC`]); записи каталога по `(pack, sid)`.
pub async fn load_doc(db: &Db, doc: &serde_json::Value) -> DbResult<Input> {
    decode(db.one_json(&doc_query(), "ch", doc.clone()).await?)
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
