-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
-- spell_system назначается в 05_refs (справочник ссылается на пак).
UPSERT pack:basic SET name = 'Basic (placeholder)', abbrev = 'B', kind = 'official', priority = 0;
UPSERT pack:homebrew SET name = 'Homebrew (test)', abbrev = 'HB', kind = 'homebrew', priority = 100;
