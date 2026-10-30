-- Новые справочные значения под структуру схемы. verified = false — сверить Мастеру.
-- Урон от ST вне таблицы: значения из комментария Мастера в 05_refs (B15: +1d за каждые полные 10 ST сверх 100).
UPSERT ref_rule:st_damage_lookup SET code = 'st_damage_lookup', text = 'floor', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = false,
	note = 'ТОЛКОВАНИЕ: правило выбора строки для ST между строками таблицы в книге не дано';
UPSERT ref_rule:st_damage_extra_st SET code = 'st_damage_extra_st', value = 10, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true,
	note = 'выше последней строки ref_st_damage: шаг ST';
UPSERT ref_rule:st_damage_extra_dice SET code = 'st_damage_extra_dice', value = 1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true,
	note = 'кубиков к thr и sw за каждый полный шаг st_damage_extra_st';
-- Самоконтроль «N/A»: броска нет, недостаток действует всегда. cost_mult — заглушка.
UPSERT ref_self_control:always SET code = 'always', kind = 'always', cost_mult = 1.0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 121 }], verified = false,
	note = 'в книге нет множителя для N/A: стоимость недостатка без самоконтроля уже базовая; 1.0 — толкование';
