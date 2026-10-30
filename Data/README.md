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
    ├── tests/suites/        # smoke, ref_integrity, translation, prereq, spells, features, character
    └── setup.surql          # служебные таблицы SurrealKit
```

## Команды (запускать из `Data/`)
```sh
cp .env.example .env
surreal start --user root --pass root memory   # dev-БД в памяти
surrealkit sync --allow-empty-prune            # схема = файлы
surrealkit seed
surrealkit test
```
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
Полнотекстовый поиск — по `translation.text` (анализаторы `gurps_en`, `gurps_ru`).
`fn::tr(rec, 'ru', 'name')` — текст с откатом на `en`. Удаление записи каскадно удаляет её переводы.
Файлы `.ftl` будут генерироваться из этой таблицы.

## Каталог
- **skill**: одна запись на навык; `spec_mode` (`none|required|optional|tl`) и список
  `specializations` (sid). Выбранная специализация — на ребре `has_skill.spec`
  (обязательна при `required`, должна быть из списка).
- **technique**: `parent_skill`, `parent_spec`, `difficulty` (только `for_techniques`),
  `derived_from` (шаблон `attack_mode` или техника — приёмы MA), `attack_delta`.
- **trait**: `kind`, `cost`, `self_control`, `categories`, `modifiers_allowed`, `features`, `prereqs`.
- **attack_mode**: шаблоны атак; `natural = true` — безоружные (Punch, Kick, Bite).
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
`damage_bonus`, `cost_reduction`, `reaction_bonus`; общие ключи `op` (`add`|`set`) и `per_level`.
Цели — только ссылки (проверяются `fn::feature_ok`). Примеры: бонус навыка к урону на кубик
(`damage_bonus { skill, amount, per_die: true }`), когти — замена типа урона удара
(`damage_bonus { op: 'set', attack_mode, damage_type }`). Ребро `affects_skill` удалено (талант = `skill_bonus`).

## Персонаж
- `character`: только ссылки и выбор игрока: `tl`, `mana_level`, `attributes` — купленные уровни
  `[{ attribute: ref_attribute:ST, levels: 1 }]` (без повторов), лимиты недостатков/причуд.
- Рёбра: `has_skill { points, spec, tl }`, `has_technique { points }`, `has_spell { points }`,
  `has_trait { level, modifiers }`, `owns_item { qty, equipped, location, contained_in }`.
  `contained_in` — другое ребро `owns_item` того же персонажа, чей предмет имеет `container`.
- `point_ledger`: журнал очков (`award` > 0, `spend` < 0, `refund`, `adjust`; `target`, `session`, `note`).
  Итог — сумма `delta`.

## Ссылочная целостность
- Каждая ссылка проверяется (`ASSERT record::exists`, `fn::ref_ok/refs_ok/rec_of`), включая
  вложенные поля, дерево пререквизитов и features.
- Удаление используемой записи (справочника, пака, цели пререквизита/эффекта) запрещено:
  вычисляемое поле `_refs` с `REFERENCE ON DELETE REJECT`.
- ASSERT не выполняется для отсутствующего значения (NONE), поэтому условная обязательность
  (owner у custom-пака, shield у щита, spec у required-навыка) — через `DEFINE EVENT ... THROW`.

## Сиды
Только заглушки (`verified = false`, урон/стоимость = `PLACEHOLDER`), нужные для тестов и примеров.
Наполнение реальными данными — отдельная задача (Мастер).
