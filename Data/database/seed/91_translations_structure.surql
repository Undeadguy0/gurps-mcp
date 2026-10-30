-- Названия новых справочников (книги, условия ран, планы тела, «N/A» самоконтроля). Тексты — сверить Мастеру.
DELETE translation WHERE record.tb() IN ['ref_book', 'ref_wound_condition', 'ref_body_plan'] OR record = ref_self_control:always;
FOR $r IN [
	[ref_book:B, 'en', 'Basic Set'], [ref_book:B, 'ru', 'Базовые правила'],
	[ref_book:M, 'en', 'Magic'], [ref_book:M, 'ru', 'Магия'],
	[ref_book:SC, 'en', 'Skill Categories'], [ref_book:SC, 'ru', 'Категории навыков'],
	[ref_wound_condition:narrow_beam, 'en', 'Narrow beam'], [ref_wound_condition:narrow_beam, 'ru', 'Узкий луч'],
	[ref_body_plan:humanoid, 'en', 'Humanoid'], [ref_body_plan:humanoid, 'ru', 'Гуманоид'],
	[ref_self_control:always, 'en', 'N/A'], [ref_self_control:always, 'ru', 'Без броска (всегда)']
] {
	CREATE translation SET record = $r[0], lang = $r[1], field = 'name', text = $r[2];
};
