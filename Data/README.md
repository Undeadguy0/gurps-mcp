# База GURPS 4e (SurrealDB + SurrealKit)

Схема спроектирована «от БД» (DB-first): без слоёв совместимости со старым форматом,
gurps-sheet и MCP-сервер подстраиваются под неё. Производные величины (уровни навыков,
HP/Speed, итоговый урон безоружных атак, сумма очков) считаются **в коде**, а не в БД.

## Структура
```
Data/
├── surrealkit.toml          # конфиг SurrealKit
├── .env.example             # подключение: скопировать в .env
├── examples.surql           # примеры запросов
└── database/
    ├── schema/              # порядок загрузки = алфавитный по пути
    │   ├── 00_common/       # analyzers, functions, prereq (дерево), feature (эффекты), pack, translation
    │   ├── 10_ref/          # справочники ref_* (урон, зоны, LC/TL, атрибуты, сложность, магия, ...)
    │   ├── 20_catalog/      # skill, technique, trait, modifier, attack_mode, item, spell
    │   ├── 30_relations/    # defaults_to, attr_default, excludes, alternative_of
    │   └── 40_character/    # character, рёбра has_*/owns_item, point_ledger
    ├── seed/                # сиды-ЗАГЛУШКИ (verified = false), не данные GURPS
    ├── tests/suites/        # smoke, ref_integrity, translation, prereq, spells, features, attack, character, calc_refs
    └── setup.surql          # служебные таблицы SurrealKit
```

## Команды (запускать из `Data/`)
Требуется **SurrealDB ≥ 3.3** (проверено на 3.3.5): в 3.1.x вычисляемое `_refs` (VALUE) считается
раньше подстановки `pack`/DEFAULT, и вставка падает.
```sh
cp .env.example .env
surreal start --user root --pass root memory   # dev-БД в памяти
surrealkit sync                                # схема = файлы
surrealkit seed
surrealkit test
```
`--allow-empty-prune` нужен только при пустом каталоге схемы; `surrealkit seed --force`
(повторный прогон уже применённых сидов) есть только в SurrealKit 1.0 (в 0.7 — пересоздать БД).
Переход со старой схемы (строковые id, name_ru) не поддерживается: dev-БД пересоздать
(`REMOVE DATABASE gurps;`), затем sync + seed.

## Идентификаторы
- Каталог (`skill`, `technique`, `trait`, `modifier`, `attack_mode`, `item`, `spell`): id — массив
  `['pack', 'sid']`, например `skill:['basic', 'broadsword']`. Поля `pack` и `sid` вычисляются из id.
- Override: та же `sid` в другом паке. Побеждает пак с большим `priority`:
  `fn::effective('skill', 'broadsword')`. Полей `override_of` и ребра `overrides` больше нет.
- Справочники `ref_*`: id = code (`ref_damage_type:cut`, `ref_tl:TL3`). Неизвестный код — ошибка.
- Пак `kind = 'custom'` с `owner = character:...` — личные записи персонажа (например,
  `trait:['custom_hero', 'oath']`).

## Переводы (`translation`)
Нет полей `name`/`name_ru`/`name_en` ни в каталоге, ни в справочниках. Все тексты — в
`translation { record, lang, field, text }`, уникальный индекс `(record, lang, field)`.
`field`: `name`, `description`, `notes`, `spec.<sid>` (название специализации).
Полнотекстовый поиск — по вычисляемым полям, раздельно по языку: `text_ru` (индекс `translation_ft_ru`,
анализатор `gurps_ru`, snowball russian — «специализации» находит «специализация») и `text_en`
(`translation_ft_en`, `gurps_en`). Для прочих языков поля `NONE`, полнотекстового индекса нет.
Запрос: `WHERE text_ru @1@ $q ORDER BY search::score(1) DESC`.
`fn::tr(rec, 'ru', 'name')` — текст с откатом на `en`. Удаление записи каскадно удаляет её переводы.
Файлы `.ftl` будут генерироваться из этой таблицы.

## Каталог
- **skill**: одна запись на навык; `spec_mode` (`none|required|optional|tl`) и список
  `specializations` (sid). Выбранная специализация — на ребре `has_skill.spec`
  (обязательна при `required`, должна быть из списка).
- **technique**: `parent_skill`, `parent_spec`, `difficulty` (только `for_techniques`),
  `derived_from` (шаблон `attack_mode` или техника — приёмы MA), `attack_delta`.
- **trait**: `kind`, `cost`, `self_control`, `categories`, `modifiers_allowed`, `features`, `prereqs`.
- **attack_mode**: шаблоны атак; `natural = true` — безоружные (Punch, Kick, Bite). Общие поля атаки
  (и у `item.attack_modes[*]`): `defaults [{ attribute | skill, mod }]`, `skill_modifier`,
  `skill_damage_bonus { min_relative, per_die }`, `technique`.
- **item**: `category` = `melee|ranged|armor|shield|gear`; `attack_modes` (со ссылкой `skill`),
  `armor`, `shield { db, parry, dr, hp }` (обязателен только у щитов), `container { capacity_weight }`, `features`.
- **spell**: `colleges[]` (≥1), `classes[]` (≥1 класса с `role = 'main'`, модификаторы вроде Resisted —
  `role = 'modifier'`), `difficulty` H/VH, `resist` (атрибут), `casting_time`/`duration`/`cost`/`maintain`
  (сырой текст `raw` + простые типизированные поля), `prereqs`. Пак заклинания обязан иметь
  `spell_system` (ref_spell_system); `spell.spell_system` вычисляется из пака.
- Справочники магии: `ref_college`, `ref_spell_class`, `ref_spell_system`, `ref_mana_level`.

## Дерево пререквизитов (`prereqs`)
Одно и то же у skill, technique, trait, spell (вместо свободной формы и ребра `requires`):
```
{ type: 'and' | 'or', of: [...] }
{ type: 'spell', spell }            { type: 'spells_from_college', college, count }
{ type: 'spells_any', count }       { type: 'magery', level }
{ type: 'attribute', attribute, min }  { type: 'advantage', trait, level? }
{ type: 'skill', skill, spec?, min? }  { type: 'technique', technique, min? }
```
Проверка `fn::prereq_ok` (рекурсивно): тип узла, допустимые ключи, существование ссылок
нужной таблицы. Выполнение пререквизитов вычисляет код.

## Эффекты (`features`)
У trait, skill, technique, item. Виды (`kind`): `attribute_bonus`, `skill_bonus`, `dr_bonus`,
`damage_bonus`, `cost_reduction`, `reaction_bonus`, `defense_bonus { amount, defenses?: [dodge|parry|block] }`
(без `defenses` — ко всем защитам), `lifting_st_bonus`, `striking_st_bonus` (`{ amount }`, только `add`),
`spell_bonus { amount, college? | spell? }` (без цели — ко всем заклинаниям); общие ключи `op` (`add`|`set`) и `per_level`.
Цели — только ссылки (проверяются `fn::feature_ok`). Примеры: бонус навыка к урону на кубик
(`damage_bonus { skill, amount, per_die: true }`), когти — замена типа урона удара
(`damage_bonus { op: 'set', attack_mode, damage_type }`). Ребро `affects_skill` удалено (талант = `skill_bonus`).

## Персонаж
- `character`: только ссылки и выбор игрока: `tl`, `mana_level`, `attributes` — купленные уровни
  `[{ attribute: ref_attribute:ST, levels: 1 }]` (без повторов; `levels` — целое число шагов
  `ref_attribute.step`, у Speed 1 уровень = +0.25), лимиты недостатков/причуд, текущие `current_hp`/`current_fp`.
- Рёбра: `has_skill { points, spec, tl, note }`, `has_technique { points, note }`, `has_spell { points, note }`,
  `has_trait { level, modifiers, note }`, `owns_item { qty, equipped, location, contained_in, note }`.
  `contained_in` — другое ребро `owns_item` того же персонажа, чей предмет имеет `container`.
- `point_ledger`: журнал очков (`award` > 0, `spend` < 0, `refund`, `adjust`; `target`, `session`, `note`).
  Итог — сумма `delta`.

## Справочники расчёта (gurps-mcp `compute_character`)
Все константы — в БД, в коде их нет:
- `ref_attribute`: `base_value` (база основного атрибута), `base_from` + `base_divisor` (база вторичного =
  сумма итоговых значений `base_from` / делитель), `cost_per_level`, `step`.
- `ref_difficulty`: `base_mod` (уровень при минимальных очках относительно атрибута), `technique_costs`
  (цена каждого следующего уровня техники, последний повторяется).
- `ref_skill_cost { points, level_offset }` — таблица очки -> прибавка к уровню;
  сверх последней строки — +1 за каждые `ref_rule:skill_cost_extra_points`.
- `ref_st_damage { st, thr { dice, mod }, sw { dice, mod } }` — урон от ST. Вне строк таблицы:
  между строками — режим `ref_rule:st_damage_lookup` (`text`, сейчас `floor` = ближайшая строка снизу);
  выше последней — +`st_damage_extra_dice` кубиков к thr и sw за каждые полные `st_damage_extra_st` ST.
- `ref_rule { code, value | text }`: `basic_lift_divisor`, `basic_lift_round_min`, `parry_base`,
  `parry_skill_divisor`, `limitation_floor_percent`, `skill_cost_extra_points`, `spell_attribute`, `magery_trait`.
- `ref_hit_location`: `to_hit_mod`, `paired`, `extra_dr`, `knockdown_mod`, `knockdown_damage_types`,
  `wound_mults[] { damage_type, mult, condition?: ref_wound_condition, note }`. Броски 3d6 локации
  зависят от плана тела, поэтому лежат не в локации, а в `ref_body_plan_location`.
- `ref_body_plan { code, is_default }` (ровно один default) и `ref_body_plan_location`
  (id `[plan, location]`, `rolls: [[от, до], ...]`, пустой — только прицельно; `to_hit_mod` — переопределение;
  диапазоны одного плана не пересекаются). `character.body_plan` — NONE = план по умолчанию.
- `ref_damage_modifier.value_kind`: `none | dice | number | multiplier | divisor | text`.
- `ref_self_control.kind`: `roll` (нужен `roll` 3..18) или `always` (без броска).
- `ref_book { code, note }`: книги; `source_refs[].book` — запись `ref_book:B`, не строка.
`rules.negative_rounding` листа в схему не добавлен: это опция стола, а не факт персонажа.

## Ссылочная целостность
- Каждая ссылка проверяется (`ASSERT record::exists`, `fn::ref_ok/refs_ok/rec_of`), включая
  вложенные поля, дерево пререквизитов и features.
- Удаление используемой записи (справочника, пака, цели пререквизита/эффекта) запрещено:
  вычисляемое поле `_refs` с `REFERENCE ON DELETE REJECT`.
- ASSERT не выполняется для отсутствующего значения (NONE), поэтому условная обязательность
  (owner у custom-пака, shield у щита, spec у required-навыка) — через `DEFINE EVENT ... THROW`.

## Сиды
`00_packs`, `01_books`, `04_wound_conditions`, `05_refs` (данные Мастера), `06_body_plans`,
`07_refs_pending` (новые правила, `verified = false`), каталог 10..50, переводы 90/91.
Каталог и персонажи — заглушки (`verified = false`, урон/стоимость = `PLACEHOLDER`, числа справочников расчёта — фиктивные), нужные для тестов и примеров.
Наполнение реальными данными — отдельная задача (Мастер).
