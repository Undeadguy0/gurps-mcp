-- Вспомогательные функции.

-- Победитель среди записей с одинаковым name: запись из пака с наибольшим priority.
-- Пример: fn::effective('skill', 'Broadsword')
DEFINE FUNCTION OVERWRITE fn::effective($tb: string, $name: string) {
	RETURN (SELECT *, pack.priority AS _prio FROM type::table($tb)
		WHERE name = $name ORDER BY _prio DESC LIMIT 1)[0];
};

-- Проверки ссылочной целостности (используются в ASSERT).
-- Одна необязательная ссылка: NONE или существующая запись.
DEFINE FUNCTION OVERWRITE fn::ref_ok($v: any) -> bool {
	RETURN $v = NONE OR (type::is_record($v) AND record::exists($v));
};
-- Массив ссылок: NONE/пустой или все записи существуют.
DEFINE FUNCTION OVERWRITE fn::refs_ok($v: any) -> bool {
	RETURN $v = NONE OR (type::is_array($v) AND $v.all(|$r| type::is_record($r) AND record::exists($r)));
};
