-- Книга-источник (B, M, SC, ...). Ссылка source_refs[*].book во всех справочниках и каталоге.
-- id = code (ref_book:B); название книги — translation field = 'name'.
DEFINE TABLE OVERWRITE ref_book SCHEMAFULL;
DEFINE FIELD OVERWRITE code ON ref_book TYPE string ASSERT $value = record::id($this.id) AND string::matches($value, '^[A-Za-z0-9]+$');
DEFINE FIELD OVERWRITE pack ON ref_book TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE note ON ref_book TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_book TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_book_code ON ref_book FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_book TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];
