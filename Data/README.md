# База GURPS 4e (SurrealDB + SurrealKit)

## Структура
```
Data/
├── surrealkit.toml          # конфиг SurrealKit (переменные шаблонов, typegen)
├── .env.example             # подключение: скопировать в .env
├── examples.surql           # примеры запросов
└── database/
    ├── schema/              # желаемое состояние схемы (по темам)
    │   ├── common/analyzers.surql   # анализаторы FTS (en, ru)
    │   ├── common/functions.surql   # fn::effective — победитель override
    │   ├── pack.surql, skill.surql, technique.surql, trait.surql,
    │   ├── modifier.surql, item.surql, relations.surql, character.surql
    ├── seed/                # сиды (идемпотентны, отслеживаются по хешу)
    ├── rollouts/            # манифесты миграций для прода (rollout plan)
    ├── snapshots/           # снимки схемы для rollout
    ├── tests/               # декларативные тесты (surrealkit test)
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
