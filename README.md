# gurps-mcp

MCP-сервер и общая база правил GURPS 4e на SurrealDB: схема каталога (навыки, черты, заклинания,
предметы, справочники правил), расчёт персонажа на чистом Rust и инструменты для ИИ-ассистентов
по протоколу MCP. Это бэкенд-часть для редактора листа
[gurps-sheet](https://github.com/Undeadguy0/gurps-sheet): тот берёт отсюда крейты `gurps-db`,
`gurps-calc` и `sheet-bridge`.

> GURPS — торговая марка Steve Jackson Games. Проект неофициальный. Сиды в `Data/database/seed` —
> заглушки (`verified = false`) с названиями, стоимостью и ссылками на страницы, без текста правил.

## Что внутри

- **Схема БД «от БД» (DB-first)** в `Data/database/schema` для SurrealKit: паки с приоритетами
  (`fn::effective` выбирает действующую запись), записи с id `['pack', 'sid']`, таблица переводов
  с полнотекстовыми индексами по языкам, дерево пререквизитов, типизированные эффекты (features),
  справочники правил `ref_*` с проверкой ссылочной целостности, персонаж с рёбрами
  `has_skill/has_trait/…/owns_item` и журналом очков.
- **Расчёт персонажа** (`crates/gurps-calc`): чистые функции, все константы правил приходят из
  справочников БД. Если константы нет, значение — `null`, а её имя попадает в `missing_constants`.
- **MCP-сервер** (`rmcp`, транспорт stdio) с инструментами:
  - `search { query, table?, lang = "ru", limit = 20 }` — полнотекст по переводам со стеммингом
    (`gurps_ru`, `gurps_en`);
  - `get_entry { table, sid, pack? }` — запись с учётом приоритета паков и переводы;
  - `list_packs {}` — паки по убыванию приоритета;
  - `get_character { id, lang = "ru" }` — персонаж со всеми рёбрами;
  - `compute_character { id }` — атрибуты и производные, уровни навыков/техник/заклинаний, атаки
    и защиты, нагрузка, сумма очков против журнала, нарушения пререквизитов;
  - `check_prereqs { character, table, sid }` — пререквизиты записи каталога для персонажа.

  `table`: `skill | trait | item | spell | technique`.
- **sheet-bridge** (`tools/sheet-bridge`) — двусторонний конвертер форматов gurps-sheet
  (TOML-плагины, `.ftl`, файл персонажа) ↔ БД с отчётом о потерях и режимом `--dry-run`
  в отменяемой транзакции. Подробности — в [`tools/sheet-bridge/README.md`](tools/sheet-bridge/README.md).

## Стек

- Rust (edition 2024 для сервера и крейтов, 2021 для sheet-bridge), `tokio`, `rmcp`, `schemars`.
- SurrealDB **≥ 3.3** (проверено на 3.3.5, клиент `surrealdb = 3.3.5`), миграции и тесты схемы —
  [SurrealKit](https://github.com/surrealdb/surrealkit).

## Структура

```text
src/                    MCP-сервер: server.rs (инструменты), tools/, calc/loader.rs (загрузка входа расчёта)
crates/gurps-calc/      расчёт: attributes, skills, attacks, features, prereq, points,
                        st_damage, encumbrance, equipment
crates/gurps-db/        общий слой SurrealDB: подключение (env SURREALDB_*), вход расчёта,
                        чтение каталога, feature testkit (временная БД для тестов)
tools/sheet-bridge/     конвертер gurps-sheet <-> БД (CLI)
tests/                  интеграционные тесты сервера (инструменты, расчёт, урон от ST, stdio-smoke)
Data/
  database/schema/      схема: 00_common, 10_ref, 20_catalog, 30_relations, 40_character
  database/seed/        сиды-заглушки и переводы
  database/tests/       тесты схемы SurrealKit (smoke, ref_integrity, prereq, spells, attack, ...)
  .env.example          параметры подключения
  examples.surql        примеры запросов
```

Подробное описание схемы и идентификаторов — в [`Data/README.md`](Data/README.md).

## Сборка и запуск

```sh
# 1. БД в памяти, схема и сиды (команды SurrealKit — из Data/)
surreal start --user root --pass root memory
cd Data && cp .env.example .env && surrealkit sync && surrealkit seed && cd ..

# 2. MCP-сервер по stdio
SURREALDB_HOST=ws://127.0.0.1:8000 SURREALDB_NAMESPACE=gurps SURREALDB_NAME=gurps \
SURREALDB_USER=root SURREALDB_PASSWORD=root cargo run --release
```

`root/root` — учётка локальной dev-БД из примера, не секрет. Логи пишутся в stderr (`RUST_LOG=debug`).
Сервер подключается к MCP-клиенту как stdio-команда (путь к собранному бинарнику `gurps-mcp`
и переменные `SURREALDB_*`).

### Тесты

```sh
cargo test --workspace                        # Rust: поднимает временную SurrealDB сам
cargo clippy --workspace --all-targets -- -D warnings
cd Data && surrealkit test                    # тесты схемы на запущенной БД
```

Тесты с БД запускают `surreal start memory` на свободном порту и грузят схему и сиды из
`GURPS_DB_DIR` (по умолчанию `Data/database`). Нужны `surreal` и `surrealkit` в `PATH`
(или `SURREAL_BIN`, `SURREALKIT_BIN`); без них такие тесты пропускаются. Тесты round-trip в
sheet-bridge ищут checkout gurps-sheet в `$GURPS_SHEET_DIR` или `../gurps-sheet` и без него
тоже пропускаются.

## Использование из gurps-sheet

gurps-sheet подключает крейты как git-зависимости этого репозитория:

```toml
gurps-db = { git = "https://github.com/Undeadguy0/gurps-mcp", branch = "main" }
sheet-bridge = { git = "https://github.com/Undeadguy0/gurps-mcp", branch = "main" }
```

`testkit` из `gurps-db` находит схему и сиды относительно своего крейта, поэтому тесты gurps-sheet
работают и с git-checkout'ом из кэша Cargo.

## Статус

Версия 0.1.0, в разработке. Сделано (по истории коммитов): схема и справочники, сиды-заглушки с
переводами, MCP-сервер с поиском и расчётом, вынос расчёта в `gurps-calc` (этап 1), нагрузка и
константы Dodge (этап 2), общий слой `gurps-db` (этап 3), нагрузка, защиты, Block, опции правил,
встроенные определения и строки атрибутов (этап 4).

Дальше (по `docs/plan-stage4.md` в gurps-sheet): строки пояснений расчёта (навыки, урон, атаки,
защиты, стоимость черт) в `gurps-calc`, сиды безоружных атак punch/kick и связь «приём → атака».
Сиды — заглушки; данные с проверенными ссылками на страницы ещё не внесены.

## Как это сделано

- **DB-first:** схема первична, сервер, расчёт и лист подстраиваются под неё; решения
  зафиксированы в [`Data/README.md`](Data/README.md).
- **Тесты на трёх уровнях:** тесты схемы SurrealKit (`Data/database/tests/suites`: целостность
  справочников, пререквизиты, переводы, атаки, нагрузка и др.), unit-тесты `gurps-calc`, интеграционные
  тесты сервера и sheet-bridge на временной БД в памяти (`testkit`), включая stdio-smoke MCP.
- **Ожидания из данных:** тесты расчёта урона от ST и compute выводят ожидания из справочников БД,
  а не из захардкоженных таблиц.
- **Поэтапная разработка:** коммиты подписаны по этапам (Этап 1–4), план этапов ведётся в
  gurps-sheet (`docs/plan-stage3.md`, `docs/plan-stage4.md`).

CI пока не настроен.

## Лицензия

[GNU General Public License v3.0](LICENSE) (`GPL-3.0-only`).
