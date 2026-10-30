-- Модификатор черты: усиление (enh) / ограничение (lim).
-- id = modifier:['pack', 'sid'] (sid и pack вычисляются из id); одинаковый sid в разных паках = override (побеждает больший pack.priority).
-- Названия/описания — в таблице translation (field = 'name' | 'description' | 'notes').
DEFINE TABLE OVERWRITE modifier SCHEMAFULL;
DEFINE FIELD OVERWRITE id ON modifier TYPE array<string, 2> ASSERT fn::id_ok($value);
DEFINE FIELD OVERWRITE sid ON modifier TYPE string VALUE record::id($this.id)[1] READONLY;
DEFINE FIELD OVERWRITE pack ON modifier TYPE record<pack> VALUE type::record('pack', record::id($this.id)[0]) ASSERT record::exists($value);
DEFINE FIELD OVERWRITE source_refs ON modifier TYPE array<object> DEFAULT [];
DEFINE FIELD OVERWRITE source_refs[*].book ON modifier TYPE string;
DEFINE FIELD OVERWRITE source_refs[*].page ON modifier TYPE option<int>;
DEFINE FIELD OVERWRITE tags ON modifier TYPE array<string> DEFAULT [];
DEFINE FIELD OVERWRITE rank ON modifier TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE verified ON modifier TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE modifier_sid ON modifier FIELDS sid;

DEFINE FIELD OVERWRITE type ON modifier TYPE string ASSERT $value IN ['enh', 'lim'];
DEFINE FIELD OVERWRITE value ON modifier TYPE object;
DEFINE FIELD OVERWRITE value.kind ON modifier TYPE string ASSERT $value IN ['%', 'pts', 'per_level'];
DEFINE FIELD OVERWRITE value.amount ON modifier TYPE number;
DEFINE FIELD OVERWRITE levels ON modifier TYPE option<int>;
DEFINE FIELD OVERWRITE incompatible ON modifier TYPE array<record<modifier>> DEFAULT [] ASSERT fn::refs_ok($value);
DEFINE FIELD OVERWRITE _refs ON modifier TYPE array<record> REFERENCE ON DELETE REJECT VALUE [pack];
