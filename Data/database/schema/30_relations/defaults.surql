-- Дефолты навыков.
-- skill/technique -> skill. mod: штраф (-5), spec: от конкретной специализации, any_spec, cap: потолок.
DEFINE TABLE OVERWRITE defaults_to TYPE RELATION IN skill | technique OUT skill SCHEMAFULL;
DEFINE FIELD OVERWRITE mod ON defaults_to TYPE int;
DEFINE FIELD OVERWRITE spec ON defaults_to TYPE option<string> ASSERT $value = NONE OR $value IN $this.out.specializations;
DEFINE FIELD OVERWRITE from_spec ON defaults_to TYPE option<string>;
DEFINE FIELD OVERWRITE any_spec ON defaults_to TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE cap ON defaults_to TYPE option<int>;

-- Дефолт от атрибута (DX-5 и т.п.).
DEFINE TABLE OVERWRITE attr_default SCHEMAFULL;
DEFINE FIELD OVERWRITE skill ON attr_default TYPE record<skill> REFERENCE ON DELETE CASCADE ASSERT record::exists($value);
DEFINE FIELD OVERWRITE attribute ON attr_default TYPE record<ref_attribute> REFERENCE ON DELETE REJECT ASSERT record::exists($value);
DEFINE FIELD OVERWRITE mod ON attr_default TYPE int;
DEFINE INDEX OVERWRITE attr_default_uniq ON attr_default FIELDS skill, attribute UNIQUE;
