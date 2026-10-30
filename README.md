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
- `search { query, table?, lang = "ru", limit = 20 }` — полнотекст по `translation.text`, откат на подстроку.
- `get_entry { table, sid, pack? }` — запись (по приоритету паков через `fn::effective`, либо из пака) + переводы.
- `list_packs {}` — паки по убыванию priority.
- `get_character { id, lang = "ru" }` — персонаж + рёбра has_skill/has_technique/has_spell/has_trait/owns_item.

`table`: `skill | trait | item | spell | technique`.

## Тесты
`cargo test` поднимает `surreal start memory`, грузит схему и сиды из `GURPS_DB_DIR`
(по умолчанию `../mcp-gurps/Data/database`); нет схемы или `surreal` — тесты с БД пропускаются.
