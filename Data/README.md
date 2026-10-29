# База GURPS 4e (SurrealDB + SurrealKit)

## Структура
```
Data/
├── surrealkit.toml          # конфиг SurrealKit (переменные шаблонов, typegen)
├── .env.example             # подключение: скопировать в .env
├── examples.surql           # примеры запросов
└── database/
    ├── schema/              # желаемое состояние схемы; порядок загрузки = алфавитный по пути
    │   ├── 00_common/       # analyzers, functions (fn::effective, fn::ref_ok, fn::refs_ok), pack
    │   ├── 10_ref/          # справочники ref_* (типы урона, зоны, LC/TL, атрибуты, ...)
    │   ├── 20_catalog/      # skill, technique, trait, modifier, item
    │   ├── 30_relations/    # рёбра каталога (defaults_to, requires, overrides, ...)
    │   └── 40_character/    # персонажи и их рёбра
    ├── seed/                # сиды (идемпотентны, отслеживаются по хешу)
    ├── rollouts/            # манифесты миграций для прода (rollout plan)
    ├── snapshots/           # снимки схемы для rollout
    ├── tests/               # тесты (surrealkit test): smoke + ref_integrity
    └── setup.surql          # служебные таблицы SurrealKit
```
Заклинания пока отложены. Сиды — неполные и НЕ сверены с книгами (страницы = NONE).

## Команды (запускать из `Data/`)
```sh
cp .env.example .env
surreal start --user root --pass root memory        # dev-БД в памяти
surrealkit sync            # dev: схема = файлы (удалённое тоже удаляется)
surrealkit sync --watch    # dev: следить за изменениями
surrealkit seed            # залить сиды (только новые/изменённые)
surrealkit test            # тесты
```
## Прод (rollout)
```sh
surrealkit rollout baseline                 # один раз для существующей БД
surrealkit rollout plan --name <имя>        # манифест в database/rollouts — закоммитить
surrealkit rollout up                       # применить
surrealkit rollout up --complete            # завершить (деструктивная фаза)
surrealkit rollout rollback <id>            # откат
```
## Принципы
- У каждой сущности: name, name_ru, pack, source_refs, tags, rank, notes, override_of.
- Override: запись хоумбрю-пака указывает override_of и ребро `overrides`; побеждает пак с большим priority (`fn::effective('skill', 'Broadsword')`).
- Сырые строки из книг (`*_raw`, с † ‡ * U F) хранятся рядом с разобранными полями.
- Персонажи: `character` + рёбра has_skill / has_trait / has_technique / owns_item.

## Справочники (ref_*)
Таблицы: ref_damage_type, ref_damage_modifier, ref_hit_location (она же места брони), ref_weapon_note,
ref_lc, ref_tl, ref_trait_category, ref_self_control, ref_attribute, ref_difficulty, ref_skill_category.
Общие поля: code (= id записи, уникален), name_ru, name_en, pack, source_refs, note, verified (по умолчанию false).
Ссылка пишется как `ref_damage_type:cut`, `ref_lc:LC4`, `ref_tl:TL3`, `ref_self_control:cr12`.

Сейчас в `seed/05_refs.surql` — только заглушки, нужные для остальных сидов (verified = false).
Наполнение реальными данными — отдельная задача (Мастер).

### Проверки
- Каждая ссылка проверяется на существование: `ASSERT record::exists($value)` / `fn::ref_ok` / `fn::refs_ok`
  (включая вложенные поля в FLEXIBLE-объектах: `attack_modes[*].damage.type`, `.damage.modifiers`, `.notes`,
  `armor.locations[*].loc`, `armor.locations[*].dr_vs[*].damage_type`).
- Удаление используемой справочной записи запрещено: вычисляемое поле `_refs` (все ссылки записи)
  с `REFERENCE ON DELETE REJECT` (в SurrealDB 3 REFERENCE работает только на корневых полях).
- `code` должен совпадать с id; LC 0–4, TL 0–12; техника — только сложность с `for_techniques = true`.

## Обновление существующей БД после перехода на папки 00_/10_/...
Файлы схемы переименованы, поэтому первый sync на старой БД откажется чистить «пропавшие» файлы. Один раз:
```sh
surrealkit sync --allow-empty-prune
surrealkit seed
```
(проверено: данные сохраняются). Старые записи со строковыми кодами (`'DX'`, `'cut'`) схема не перепроверяет —
их нужно переписать на ссылки (сиды делают это сами).
