-- Названия новых справочников (книги, условия ран, планы тела, «N/A» самоконтроля). Тексты — сверить Мастеру; новые зоны и планы тела — B552-554.
DELETE translation WHERE record.tb() IN ['ref_book', 'ref_wound_condition', 'ref_body_plan'] OR record = ref_self_control:always OR record IN [ref_hit_location:foreleg, ref_hit_location:hind_leg, ref_hit_location:mid_leg, ref_hit_location:wing, ref_hit_location:tail, ref_hit_location:extremity, ref_hit_location:brain, ref_hit_location:fin];
FOR $r IN [
	[ref_book:B, 'en', 'Basic Set'], [ref_book:B, 'ru', 'Базовые правила'],
	[ref_book:M, 'en', 'Magic'], [ref_book:M, 'ru', 'Магия'],
	[ref_book:SC, 'en', 'Skill Categories'], [ref_book:SC, 'ru', 'Категории навыков'],
	[ref_wound_condition:narrow_beam, 'en', 'Narrow beam'], [ref_wound_condition:narrow_beam, 'ru', 'Узкий луч'],
	[ref_hit_location:foreleg, 'ru', 'Передняя нога'], [ref_hit_location:foreleg, 'en', 'Foreleg'],
	[ref_hit_location:hind_leg, 'ru', 'Задняя нога'], [ref_hit_location:hind_leg, 'en', 'Hind Leg'],
	[ref_hit_location:mid_leg, 'ru', 'Средняя нога'], [ref_hit_location:mid_leg, 'en', 'Mid Leg'],
	[ref_hit_location:wing, 'ru', 'Крыло'], [ref_hit_location:wing, 'en', 'Wing'],
	[ref_hit_location:tail, 'ru', 'Хвост'], [ref_hit_location:tail, 'en', 'Tail'],
	[ref_hit_location:extremity, 'ru', 'Конечность (кисть/ступня)'], [ref_hit_location:extremity, 'en', 'Extremity'],
	[ref_hit_location:brain, 'ru', 'Мозг'], [ref_hit_location:brain, 'en', 'Brain'],
	[ref_hit_location:fin, 'ru', 'Плавник'], [ref_hit_location:fin, 'en', 'Fin'],
	[ref_body_plan:humanoid, 'ru', 'Гуманоид'], [ref_body_plan:humanoid, 'en', 'Humanoid'],
	[ref_body_plan:winged_humanoid, 'ru', 'Крылатый гуманоид'], [ref_body_plan:winged_humanoid, 'en', 'Winged Humanoid'],
	[ref_body_plan:quadruped, 'ru', 'Четвероногое'], [ref_body_plan:quadruped, 'en', 'Quadruped'],
	[ref_body_plan:winged_quadruped, 'ru', 'Крылатое четвероногое'], [ref_body_plan:winged_quadruped, 'en', 'Winged Quadruped'],
	[ref_body_plan:hexapod, 'ru', 'Шестиногое'], [ref_body_plan:hexapod, 'en', 'Hexapod'],
	[ref_body_plan:centaur, 'ru', 'Кентавр'], [ref_body_plan:centaur, 'en', 'Centaur'],
	[ref_body_plan:avian, 'ru', 'Птица'], [ref_body_plan:avian, 'en', 'Avian'],
	[ref_body_plan:vermiform, 'ru', 'Червеобразное'], [ref_body_plan:vermiform, 'en', 'Vermiform'],
	[ref_body_plan:snakeman, 'ru', 'Человек-змея'], [ref_body_plan:snakeman, 'en', 'Snake-Man'],
	[ref_body_plan:octopod, 'ru', 'Восьминогое (осьминог)'], [ref_body_plan:octopod, 'en', 'Octopod'],
	[ref_body_plan:squid, 'ru', 'Кальмар'], [ref_body_plan:squid, 'en', 'Squid'],
	[ref_body_plan:cancroid, 'ru', 'Ракообразное'], [ref_body_plan:cancroid, 'en', 'Cancroid'],
	[ref_body_plan:scorpion, 'ru', 'Скорпион'], [ref_body_plan:scorpion, 'en', 'Scorpion'],
	[ref_body_plan:ichthyoid, 'ru', 'Рыбообразное'], [ref_body_plan:ichthyoid, 'en', 'Ichthyoid'],
	[ref_body_plan:arachnoid, 'ru', 'Паукообразное'], [ref_body_plan:arachnoid, 'en', 'Arachnoid'],
	[ref_body_plan:spider_morph, 'ru', 'Паук-морф (заглушка)'], [ref_body_plan:spider_morph, 'en', 'Spider-morph (placeholder)'],
	[ref_self_control:always, 'en', 'N/A'], [ref_self_control:always, 'ru', 'Без броска (всегда)']
] {
	CREATE translation SET record = $r[0], lang = $r[1], field = 'name', text = $r[2];
};
