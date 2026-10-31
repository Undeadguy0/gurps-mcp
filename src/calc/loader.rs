//! Загрузка входных данных расчёта из БД одним запросом (ссылки -> sid/code).
use super::model::Input;
use crate::db::Db;
use crate::error::{ToolError, ToolResult};

/// Режим атаки (объект `$m`) -> форма [`super::model::AttackModeDef`].
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
    rules: (SELECT code, value, text FROM ref_rule)
}";

pub fn query() -> String {
    let natural = attack("$m", "$m.sid");
    let item = attack("$m", "string::concat('item/', $m.name ?? '')");
    format!(
        "LET $c = type::record('character', $id);
        RETURN IF !record::exists($c) {{ NONE }} ELSE {{ {{
            refs: {REFS},
            attributes: $c.attributes.map(|$a| {{ code: record::id($a.attribute), levels: $a.levels }}),
            current_hp: $c.current_hp, current_fp: $c.current_fp,
            traits: (SELECT out.sid AS sid, level, out.cost AS cost,
                        modifiers.map(|$m| {{ type: $m.modifier.type, value_kind: $m.modifier.value.kind,
                            amount: $m.modifier.value.amount, levels: $m.levels ?? $m.modifier.levels }}) AS modifiers,
                        out.features AS features, out.prereqs AS prereqs
                     FROM has_trait WHERE in = $c),
            skills: (SELECT out.sid AS sid, record::id(out.attribute) AS attribute,
                        record::id(out.difficulty) AS difficulty, points, spec,
                        out.features AS features, out.prereqs AS prereqs
                     FROM has_skill WHERE in = $c),
            techniques: (SELECT out.sid AS sid, out.parent_skill.sid AS parent_skill,
                        record::id(out.difficulty) AS difficulty, out.default_mod AS default_mod,
                        out.max_mod AS max_mod, points, out.features AS features, out.prereqs AS prereqs
                     FROM has_technique WHERE in = $c),
            spells: (SELECT out.sid AS sid, out.colleges.map(|$x| record::id($x)) AS colleges,
                        record::id(out.difficulty) AS difficulty, points, out.prereqs AS prereqs
                     FROM has_spell WHERE in = $c),
            items: (SELECT out.sid AS sid, equipped, out.features AS features,
                        (out.attack_modes ?? []).map(|$m| {item}) AS attack_modes
                     FROM owns_item WHERE in = $c),
            natural_attacks: array::distinct((SELECT VALUE sid FROM attack_mode WHERE natural = true))
                .map(|$s| fn::effective('attack_mode', $s))
                .filter(|$m| $m != NONE AND $m.natural = true)
                .map(|$m| {natural}),
            defaults: array::concat(
                (SELECT skill.sid AS skill, record::id(attribute) AS attribute, mod FROM attr_default),
                (SELECT in.sid AS skill, out.sid AS target, mod FROM defaults_to)),
            catalog_skills: (SELECT sid, record::id(attribute) AS attribute FROM skill),
            ledger: (SELECT kind, delta FROM point_ledger WHERE character = $c)
        }} }};"
    )
}

/// Входные данные расчёта для персонажа `character:<id>`.
pub async fn load(db: &Db, id: &str) -> ToolResult<Input> {
    let v = db.one(&query(), vec![("id", id.to_string())]).await?;
    if v.is_null() {
        return Err(ToolError::NotFound(format!("character:{id}")));
    }
    serde_json::from_value(v).map_err(|e| ToolError::Db(format!("разбор данных расчёта: {e}")))
}
