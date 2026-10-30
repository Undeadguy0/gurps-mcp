-- Книги-источники (ref_book). Коды — из прежних строковых source_refs.book Мастера; названия — в 91_translations_structure.
-- Сверено с титульными страницами PDF; страницы B — сквозные (Characters 1-336, Campaigns 337+).
UPSERT ref_book:B SET code = 'B', pack = pack:basic, verified = true, note = 'GURPS Basic Set, Fourth Edition: Characters + Campaigns (сквозная нумерация)';
UPSERT ref_book:M SET code = 'M', pack = pack:basic, verified = true, note = 'GURPS Magic, Fourth Edition';
UPSERT ref_book:SC SET code = 'SC', pack = pack:basic, verified = true, note = 'GURPS Skill Categories (4e)';
