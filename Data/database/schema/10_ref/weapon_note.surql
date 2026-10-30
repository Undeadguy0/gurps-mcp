-- Сноски/флаги оружия († ‡ * U F ...).
DEFINE TABLE OVERWRITE ref_weapon_note SCHEMAFULL;
-- Общие поля справочника: id = code (например ref_weapon_note:dagger); названия — в translation.
DEFINE FIELD OVERWRITE code ON ref_weapon_note TYPE string ASSERT $value = record::id($this.id);
DEFINE FIELD OVERWRITE pack ON ref_weapon_note TYPE record<pack> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON ref_weapon_note TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON ref_weapon_note TYPE record<ref_book> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs[*].page ON ref_weapon_note TYPE option<int>;
DEFINE FIELD OVERWRITE note ON ref_weapon_note TYPE option<string>;
DEFINE FIELD OVERWRITE verified ON ref_weapon_note TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE ref_weapon_note_code ON ref_weapon_note FIELDS code UNIQUE;
DEFINE FIELD OVERWRITE symbol ON ref_weapon_note TYPE string;
DEFINE FIELD OVERWRITE column ON ref_weapon_note TYPE string ASSERT $value IN ['reach', 'parry', 'st', 'shots', 'rof', 'other'];
DEFINE INDEX OVERWRITE ref_weapon_note_symbol ON ref_weapon_note FIELDS symbol, column UNIQUE;
DEFINE FIELD OVERWRITE _refs ON ref_weapon_note TYPE array<record> REFERENCE ON DELETE REJECT VALUE array::distinct(array::flatten([[pack], source_refs.map(|$s| $s.book)]));
