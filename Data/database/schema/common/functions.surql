-- Вспомогательные функции.

-- Победитель среди записей с одинаковым name: запись из пака с наибольшим priority.
-- Пример: fn::effective('skill', 'Broadsword')
DEFINE FUNCTION OVERWRITE fn::effective($tb: string, $name: string) {
	RETURN (SELECT *, pack.priority AS _prio FROM type::table($tb)
		WHERE name = $name ORDER BY _prio DESC LIMIT 1)[0];
};
