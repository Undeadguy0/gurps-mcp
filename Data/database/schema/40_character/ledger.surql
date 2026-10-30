-- Журнал очков персонажа: каждое начисление/трата. Итоги считаются в коде (math::sum(delta)).
-- kind: award (начисление), spend (покупка), refund (возврат), adjust (правка Мастера).
DEFINE TABLE OVERWRITE point_ledger SCHEMAFULL;
DEFINE FIELD OVERWRITE character ON point_ledger TYPE record<character> REFERENCE ON DELETE CASCADE ASSERT record::exists($value);
DEFINE FIELD OVERWRITE at ON point_ledger TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE kind ON point_ledger TYPE string ASSERT $value IN ['award', 'spend', 'refund', 'adjust'];
DEFINE FIELD OVERWRITE delta ON point_ledger TYPE int
	ASSERT IF $this.kind = 'award' { $value > 0 } ELSE IF $this.kind = 'spend' { $value < 0 } ELSE { true };
-- На что потрачено: запись каталога, ребро has_* или ref_attribute
DEFINE FIELD OVERWRITE target ON point_ledger TYPE option<record> ASSERT fn::ref_ok($value);
DEFINE FIELD OVERWRITE session ON point_ledger TYPE option<string>;
DEFINE FIELD OVERWRITE note ON point_ledger TYPE option<string>;
DEFINE INDEX OVERWRITE point_ledger_char ON point_ledger FIELDS character, at;
