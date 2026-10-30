-- Вспомогательные функции.

-- Действующая запись по sid: из пака с наибольшим priority. Пример: fn::effective('skill', 'sword_a')
DEFINE FUNCTION OVERWRITE fn::effective($tb: string, $sid: string) {
	RETURN (SELECT *, pack.priority AS _prio FROM type::table($tb) WHERE sid = $sid ORDER BY _prio DESC LIMIT 1)[0];
};
-- Текст перевода с откатом на 'en'. Пример: fn::tr(skill:['basic','sword_a'], 'ru', 'name')
DEFINE FUNCTION OVERWRITE fn::tr($rec: record, $lang: string, $field: string) {
	LET $all = SELECT lang, text FROM translation WHERE record = $rec AND field = $field;
	RETURN ($all.filter(|$t| $t.lang = $lang)[0] ?? $all.filter(|$t| $t.lang = 'en')[0]).text;
};
-- id каталожной записи: массив ['pack', 'sid'] из двух строк вида [a-z0-9_].
DEFINE FUNCTION OVERWRITE fn::id_ok($id: any) -> bool {
	LET $k = IF type::is_record($id) { record::id($id) } ELSE { $id };
	RETURN type::is_array($k) AND array::len($k) = 2
		AND $k.all(|$s| type::is_string($s) AND string::matches($s, '^[a-z0-9_]+$'));
};
-- Одна необязательная ссылка: NONE или существующая запись.
DEFINE FUNCTION OVERWRITE fn::ref_ok($v: any) -> bool {
	RETURN $v = NONE OR (type::is_record($v) AND record::exists($v));
};
-- Массив ссылок: NONE/пустой или все записи существуют.
DEFINE FUNCTION OVERWRITE fn::refs_ok($v: any) -> bool {
	RETURN $v = NONE OR (type::is_array($v) AND $v.all(|$r| type::is_record($r) AND record::exists($r)));
};
-- Существующая запись нужной таблицы (одной из).
DEFINE FUNCTION OVERWRITE fn::rec_of($v: any, $tbs: array) -> bool {
	RETURN type::is_record($v) AND record::tb($v) IN $tbs AND record::exists($v);
};
DEFINE FUNCTION OVERWRITE fn::opt_rec_of($v: any, $tbs: array) -> bool {
	RETURN $v = NONE OR fn::rec_of($v, $tbs);
};
DEFINE FUNCTION OVERWRITE fn::opt_num($v: any) -> bool { RETURN $v = NONE OR type::is_number($v); };
DEFINE FUNCTION OVERWRITE fn::opt_int($v: any) -> bool { RETURN $v = NONE OR type::is_int($v); };
DEFINE FUNCTION OVERWRITE fn::opt_bool($v: any) -> bool { RETURN $v = NONE OR type::is_bool($v); };
DEFINE FUNCTION OVERWRITE fn::opt_str($v: any) -> bool { RETURN $v = NONE OR type::is_string($v); };
