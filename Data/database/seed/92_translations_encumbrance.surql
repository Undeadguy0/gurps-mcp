-- Названия уровней нагрузки (ref_encumbrance, B17). Отдельный файл после 90_translations, т.к. тот очищает translation.
DELETE translation WHERE record.tb() = 'ref_encumbrance';
FOR $r IN [
	[ref_encumbrance:none, 'ru', 'Нет'], [ref_encumbrance:none, 'en', 'None'],
	[ref_encumbrance:light, 'ru', 'Лёгкая'], [ref_encumbrance:light, 'en', 'Light'],
	[ref_encumbrance:medium, 'ru', 'Средняя'], [ref_encumbrance:medium, 'en', 'Medium'],
	[ref_encumbrance:heavy, 'ru', 'Тяжёлая'], [ref_encumbrance:heavy, 'en', 'Heavy'],
	[ref_encumbrance:x_heavy, 'ru', 'Сверхтяжёлая'], [ref_encumbrance:x_heavy, 'en', 'Extra-Heavy']
] {
	CREATE translation SET record = $r[0], lang = $r[1], field = 'name', text = $r[2];
};
