# gurps-mcp

MCP-сервер (rmcp, stdio) для базы GURPS 4e в SurrealDB (схема `mcp-gurps/Data/database`).

## Запуск
```sh
surreal start --user root --pass root memory   # + surrealkit sync && surrealkit seed
SURREALDB_HOST=ws://127.0.0.1:8000 SURREALDB_NAMESPACE=gurps SURREALDB_NAME=gurps \
SURREALDB_USER=root SURREALDB_PASSWORD=root cargo run --release
```
Логи — в stderr (`RUST_LOG=debug`).

## Инструменты
- `search { query, table?, lang = "ru", limit = 20 }` — полнотекст по `translation.text_<lang>`: свой индекс
  и анализатор на язык (`gurps_ru` со стеммером — словоформы находятся; `gurps_en`). Только `ru`/`en`,
  другой язык — ошибка параметров. Отката на подстроку нет: стемминг по языку его заменяет.
- `get_entry { table, sid, pack? }` — запись (по приоритету паков через `fn::effective`, либо из пака) + переводы.
- `list_packs {}` — паки по убыванию priority.
- `get_character { id, lang = "ru" }` — персонаж + рёбра has_skill/has_technique/has_spell/has_trait/owns_item.

- `compute_character { id }` — расчёт: атрибуты и производные (Basic Lift, thr/sw от ударной ST, бонусы
  защит, текущие HP/FP), уровни навыков/техник/заклинаний, атаки (безоружные шаблоны + режимы надетых
  предметов), сумма очков против журнала, нарушения пререквизитов, `missing_constants`.
- `check_prereqs { character, table, sid }` — пререквизиты действующей записи каталога для персонажа.

`table`: `skill | trait | item | spell | technique`.

## Расчёт (`src/calc`)
Чистый Rust; `loader` одним запросом собирает данные, модули по темам: `attributes`, `skills`,
`attacks`, `features`, `prereq`, `points`. Все константы — из справочников БД (`ref_attribute`,
`ref_difficulty`, `ref_skill_cost`, `ref_st_damage`, `ref_rule`); если константы нет, соответствующее
значение `null`, а её имя попадает в `missing_constants`.
- Атрибут = база (`base_value` или сумма `base_from` / `base_divisor`; у атрибутов с целым шагом дробь
  отбрасывается) + `levels * step` + `attribute_bonus` (`op = set` заменяет).
- Навык = атрибут + `base_mod` сложности + прибавка за очки (`ref_skill_cost`, сверх таблицы — +1 за
  каждые `skill_cost_extra_points`); лучшее из этого и умолчаний (от атрибута или изученного навыка,
  без цепочек) + `skill_bonus`.
- Техника = навык-родитель + `default_mod` + шаги по `technique_costs`, не выше родителя + `max_mod`.
- Заклинание = атрибут `ref_rule:spell_attribute` + сложность + очки + `spell_bonus` (все/школа/заклинание).
- Атака: уровень — лучший из навыка, `defaults`, техники, затем `skill_modifier`; урон — thr/sw от
  ударной ST (+`striking_st_bonus`), плоские бонусы, `op = set` (тип/досягаемость: когти), бонусы
  «за кубик» (`damage_bonus.per_die`, `skill_damage_bonus` при относительном уровне >= `min_relative`)
  последними; парирование = `parry_base` + уровень / `parry_skill_divisor` + parry атаки + `defense_bonus`.
- Очки: атрибуты (`levels * cost_per_level`), черты (база/уровни, модификаторы: % суммой с нижней
  границей `limitation_floor_percent`, дроби вверх), навыки, техники, заклинания; журнал: бюджет = всё,
  кроме spend; потрачено = −сумма spend.

## Тесты
`cargo test` поднимает `surreal start memory`, грузит схему и сиды из `GURPS_DB_DIR`
(по умолчанию `../mcp-gurps/Data/database`); нет схемы или `surreal` — тесты с БД пропускаются.
