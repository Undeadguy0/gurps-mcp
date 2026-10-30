-- Планы тела (B552-555) и броски 3d6 по зонам.
-- humanoid: броски перенесены из прежних ref_hit_location.roll_range Мастера (правые/левые руки и ноги
-- объединены в arm/leg с двумя диапазонами); verified = true сохранено.
UPSERT ref_body_plan:humanoid SET code = 'humanoid', is_default = true, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 552 }], verified = true;
DELETE ref_body_plan_location WHERE plan = ref_body_plan:humanoid;
FOR $r IN [
	['eye', []], ['skull', [[3, 4]]], ['face', [[5, 5]]], ['leg', [[6, 7], [13, 14]]], ['arm', [[8, 8], [12, 12]]],
	['torso', [[9, 10]]], ['groin', [[11, 11]]], ['hand', [[15, 15]]], ['foot', [[16, 16]]], ['neck', [[17, 18]]], ['vitals', []]
] {
	CREATE type::record('ref_body_plan_location', ['humanoid', $r[0]]) SET plan = ref_body_plan:humanoid,
		location = type::record('ref_hit_location', $r[0]), rolls = $r[1], pack = pack:basic,
		source_refs = [{ book: ref_book:B, page: 552 }], verified = true;
};

-- ЗАГЛУШКИ: прочие планы тела (B553-555) без зон; состав зон, броски и новые зоны (крыло, хвост, ...) — Мастер.
FOR $c IN ['quadruped', 'hexapod', 'centaur', 'avian', 'winged_humanoid', 'winged_quadruped', 'ichthyoid',
	'vermiform', 'snakeman', 'octopod', 'squid', 'cancroid', 'scorpion', 'arachnoid', 'spider_morph'] {
	UPSERT type::record('ref_body_plan', $c) SET code = $c, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }],
		verified = false, note = 'ЗАГЛУШКА: зоны и броски не заполнены';
};
