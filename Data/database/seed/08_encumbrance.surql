-- Нагрузка (B17). Значения сверены с PDF Basic Set (стр. B17 = PDF 19). Названия — в 92_translations_encumbrance (90_translations очищает translation целиком).
-- Dodge = floor(Basic Speed) + dodge_base; при нагрузке + dodge_mod. Move = floor(Basic Move * move_mult), не ниже encumbrance_move_min; Dodge не ниже dodge_min.
UPSERT ref_encumbrance:none SET code = 'none', level = 0, max_load_bl = 1, move_mult = 1, dodge_mod = 0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true;
UPSERT ref_encumbrance:light SET code = 'light', level = 1, max_load_bl = 2, move_mult = 0.8, dodge_mod = -1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true;
UPSERT ref_encumbrance:medium SET code = 'medium', level = 2, max_load_bl = 3, move_mult = 0.6, dodge_mod = -2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true;
UPSERT ref_encumbrance:heavy SET code = 'heavy', level = 3, max_load_bl = 6, move_mult = 0.4, dodge_mod = -3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true;
UPSERT ref_encumbrance:x_heavy SET code = 'x_heavy', level = 4, max_load_bl = 10, move_mult = 0.2, dodge_mod = -4, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true;
UPSERT ref_rule:dodge_attribute SET code = 'dodge_attribute', text = 'Speed', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true, note = 'Dodge от Basic Speed без дробной части; код ref_attribute:Speed';
UPSERT ref_rule:dodge_base SET code = 'dodge_base', value = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true, note = 'Dodge = floor(Basic Speed) + 3';
UPSERT ref_rule:encumbrance_move_min SET code = 'encumbrance_move_min', value = 1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true,
	note = 'нагрузка не снижает Move ниже 1; тот же минимум 1 для Dodge — ref_rule:dodge_min';
UPSERT ref_rule:dodge_min SET code = 'dodge_min', value = 1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true, note = 'нагрузка не снижает Dodge ниже 1';
UPSERT ref_rule:move_attribute SET code = 'move_attribute', text = 'Move', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true, note = 'нагрузка умножает Basic Move; код ref_attribute:Move';
