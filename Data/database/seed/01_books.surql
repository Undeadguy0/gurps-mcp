-- Книги-источники (ref_book). Коды — из прежних строковых source_refs.book Мастера; названия — в 91_translations_structure.
-- verified = false: названия и состав книг сверяет Мастер.
UPSERT ref_book:B SET code = 'B', pack = pack:basic, verified = false, note = 'GURPS Basic Set 4e (Characters + Campaigns, сквозная нумерация)';
UPSERT ref_book:M SET code = 'M', pack = pack:basic, verified = false, note = 'GURPS Magic 4e';
UPSERT ref_book:SC SET code = 'SC', pack = pack:basic, verified = false, note = 'GURPS Skill Categories';
