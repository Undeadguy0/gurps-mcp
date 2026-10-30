-- Справочники GURPS 4e. Значения сверены с текстом PDF; source_refs = книга + печатная страница.
-- B = Basic Set, M = Magic, SC = GURPS Skill Categories. Названия — в 90_translations.

-- Атрибуты (B14-B17)
UPSERT ref_attribute:ST SET code = 'ST', kind = 'basic', base_value = 10, base_from = [], cost_per_level = 10, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 14 }], verified = true;
UPSERT ref_attribute:DX SET code = 'DX', kind = 'basic', base_value = 10, base_from = [], cost_per_level = 20, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true;
UPSERT ref_attribute:IQ SET code = 'IQ', kind = 'basic', base_value = 10, base_from = [], cost_per_level = 20, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true;
UPSERT ref_attribute:HT SET code = 'HT', kind = 'basic', base_value = 10, base_from = [], cost_per_level = 10, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true;
UPSERT ref_attribute:HP SET code = 'HP', kind = 'secondary', base_from = [ref_attribute:ST], cost_per_level = 2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_attribute:Will SET code = 'Will', kind = 'secondary', base_from = [ref_attribute:IQ], cost_per_level = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_attribute:Per SET code = 'Per', kind = 'secondary', base_from = [ref_attribute:IQ], cost_per_level = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_attribute:FP SET code = 'FP', kind = 'secondary', base_from = [ref_attribute:HT], cost_per_level = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_attribute:Speed SET code = 'Speed', kind = 'secondary', step = 0.25, base_from = [ref_attribute:DX, ref_attribute:HT], base_divisor = 4, cost_per_level = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true, note = '(DX+HT)/4 без округления; 5 очков за шаг 0.25';
UPSERT ref_attribute:Move SET code = 'Move', kind = 'secondary', base_from = [ref_attribute:Speed], cost_per_level = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 17 }], verified = true, note = 'Basic Speed без дробной части';

-- Сложность навыков (B170), цены техник (B230)
UPSERT ref_difficulty:E SET code = 'E', base_mod = 0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_difficulty:A SET code = 'A', base_mod = -1, for_techniques = true, technique_costs = [1], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true, note = 'техники: B230';
UPSERT ref_difficulty:H SET code = 'H', base_mod = -2, for_techniques = true, technique_costs = [2, 1], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true, note = 'техники: B230';
UPSERT ref_difficulty:VH SET code = 'VH', base_mod = -3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;

-- Таблица цены навыков (B170): очки -> атрибут + base_mod + level_offset
UPSERT ref_skill_cost:p1 SET code = 'p1', points = 1, level_offset = 0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p2 SET code = 'p2', points = 2, level_offset = 1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p4 SET code = 'p4', points = 4, level_offset = 2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p8 SET code = 'p8', points = 8, level_offset = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p12 SET code = 'p12', points = 12, level_offset = 4, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p16 SET code = 'p16', points = 16, level_offset = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p20 SET code = 'p20', points = 20, level_offset = 6, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p24 SET code = 'p24', points = 24, level_offset = 7, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;
UPSERT ref_skill_cost:p28 SET code = 'p28', points = 28, level_offset = 8, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true;

-- Урон от ST (B16). Выше 100: +1d к thr и sw за каждые полные 10 ST сверх 100 (B15) — ref_rule:st_damage_extra_* (07_refs_pending).
UPSERT ref_st_damage:st1 SET code = 'st1', st = 1, thr = { dice: 1, mod: -6 }, sw = { dice: 1, mod: -5 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st2 SET code = 'st2', st = 2, thr = { dice: 1, mod: -6 }, sw = { dice: 1, mod: -5 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st3 SET code = 'st3', st = 3, thr = { dice: 1, mod: -5 }, sw = { dice: 1, mod: -4 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st4 SET code = 'st4', st = 4, thr = { dice: 1, mod: -5 }, sw = { dice: 1, mod: -4 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st5 SET code = 'st5', st = 5, thr = { dice: 1, mod: -4 }, sw = { dice: 1, mod: -3 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st6 SET code = 'st6', st = 6, thr = { dice: 1, mod: -4 }, sw = { dice: 1, mod: -3 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st7 SET code = 'st7', st = 7, thr = { dice: 1, mod: -3 }, sw = { dice: 1, mod: -2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st8 SET code = 'st8', st = 8, thr = { dice: 1, mod: -3 }, sw = { dice: 1, mod: -2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st9 SET code = 'st9', st = 9, thr = { dice: 1, mod: -2 }, sw = { dice: 1, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st10 SET code = 'st10', st = 10, thr = { dice: 1, mod: -2 }, sw = { dice: 1, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st11 SET code = 'st11', st = 11, thr = { dice: 1, mod: -1 }, sw = { dice: 1, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st12 SET code = 'st12', st = 12, thr = { dice: 1, mod: -1 }, sw = { dice: 1, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st13 SET code = 'st13', st = 13, thr = { dice: 1, mod: 0 }, sw = { dice: 2, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st14 SET code = 'st14', st = 14, thr = { dice: 1, mod: 0 }, sw = { dice: 2, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st15 SET code = 'st15', st = 15, thr = { dice: 1, mod: +1 }, sw = { dice: 2, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st16 SET code = 'st16', st = 16, thr = { dice: 1, mod: +1 }, sw = { dice: 2, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st17 SET code = 'st17', st = 17, thr = { dice: 1, mod: +2 }, sw = { dice: 3, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st18 SET code = 'st18', st = 18, thr = { dice: 1, mod: +2 }, sw = { dice: 3, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st19 SET code = 'st19', st = 19, thr = { dice: 2, mod: -1 }, sw = { dice: 3, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st20 SET code = 'st20', st = 20, thr = { dice: 2, mod: -1 }, sw = { dice: 3, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st21 SET code = 'st21', st = 21, thr = { dice: 2, mod: 0 }, sw = { dice: 4, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st22 SET code = 'st22', st = 22, thr = { dice: 2, mod: 0 }, sw = { dice: 4, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st23 SET code = 'st23', st = 23, thr = { dice: 2, mod: +1 }, sw = { dice: 4, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st24 SET code = 'st24', st = 24, thr = { dice: 2, mod: +1 }, sw = { dice: 4, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st25 SET code = 'st25', st = 25, thr = { dice: 2, mod: +2 }, sw = { dice: 5, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st26 SET code = 'st26', st = 26, thr = { dice: 2, mod: +2 }, sw = { dice: 5, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st27 SET code = 'st27', st = 27, thr = { dice: 3, mod: -1 }, sw = { dice: 5, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st28 SET code = 'st28', st = 28, thr = { dice: 3, mod: -1 }, sw = { dice: 5, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st29 SET code = 'st29', st = 29, thr = { dice: 3, mod: 0 }, sw = { dice: 5, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st30 SET code = 'st30', st = 30, thr = { dice: 3, mod: 0 }, sw = { dice: 5, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st31 SET code = 'st31', st = 31, thr = { dice: 3, mod: +1 }, sw = { dice: 6, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st32 SET code = 'st32', st = 32, thr = { dice: 3, mod: +1 }, sw = { dice: 6, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st33 SET code = 'st33', st = 33, thr = { dice: 3, mod: +2 }, sw = { dice: 6, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st34 SET code = 'st34', st = 34, thr = { dice: 3, mod: +2 }, sw = { dice: 6, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st35 SET code = 'st35', st = 35, thr = { dice: 4, mod: -1 }, sw = { dice: 6, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st36 SET code = 'st36', st = 36, thr = { dice: 4, mod: -1 }, sw = { dice: 6, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st37 SET code = 'st37', st = 37, thr = { dice: 4, mod: 0 }, sw = { dice: 6, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st38 SET code = 'st38', st = 38, thr = { dice: 4, mod: 0 }, sw = { dice: 6, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st39 SET code = 'st39', st = 39, thr = { dice: 4, mod: +1 }, sw = { dice: 7, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st40 SET code = 'st40', st = 40, thr = { dice: 4, mod: +1 }, sw = { dice: 7, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st45 SET code = 'st45', st = 45, thr = { dice: 5, mod: 0 }, sw = { dice: 7, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st50 SET code = 'st50', st = 50, thr = { dice: 5, mod: +2 }, sw = { dice: 8, mod: -1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st55 SET code = 'st55', st = 55, thr = { dice: 6, mod: 0 }, sw = { dice: 8, mod: +1 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st60 SET code = 'st60', st = 60, thr = { dice: 7, mod: -1 }, sw = { dice: 9, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st65 SET code = 'st65', st = 65, thr = { dice: 7, mod: +1 }, sw = { dice: 9, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st70 SET code = 'st70', st = 70, thr = { dice: 8, mod: 0 }, sw = { dice: 10, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st75 SET code = 'st75', st = 75, thr = { dice: 8, mod: +2 }, sw = { dice: 10, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st80 SET code = 'st80', st = 80, thr = { dice: 9, mod: 0 }, sw = { dice: 11, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st85 SET code = 'st85', st = 85, thr = { dice: 9, mod: +2 }, sw = { dice: 11, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st90 SET code = 'st90', st = 90, thr = { dice: 10, mod: 0 }, sw = { dice: 12, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st95 SET code = 'st95', st = 95, thr = { dice: 10, mod: +2 }, sw = { dice: 12, mod: +2 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;
UPSERT ref_st_damage:st100 SET code = 'st100', st = 100, thr = { dice: 11, mod: 0 }, sw = { dice: 13, mod: 0 }, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 16 }], verified = true;

-- Константы расчёта
UPSERT ref_rule:basic_lift_divisor SET code = 'basic_lift_divisor', value = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true, note = 'BL = ST*ST/5';
UPSERT ref_rule:basic_lift_round_min SET code = 'basic_lift_round_min', value = 10, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 15 }], verified = true, note = 'BL от 10 и выше округляется до целого';
UPSERT ref_rule:parry_base SET code = 'parry_base', value = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 376 }], verified = true, note = 'Parry = 3 + skill/2';
UPSERT ref_rule:parry_skill_divisor SET code = 'parry_skill_divisor', value = 2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 376 }], verified = true;
UPSERT ref_rule:limitation_floor_percent SET code = 'limitation_floor_percent', value = -80, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 101 }], verified = true, note = 'чистый модификатор ниже -80% считается -80%';
UPSERT ref_rule:skill_cost_extra_points SET code = 'skill_cost_extra_points', value = 4, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 170 }], verified = true, note = 'Extra +1: +4 очка';
UPSERT ref_rule:spell_attribute SET code = 'spell_attribute', text = 'IQ', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 235 }], verified = true, note = 'заклинания IQ/Hard или IQ/VH; также M6';
UPSERT ref_rule:magery_trait SET code = 'magery_trait', text = 'magery', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 66 }], verified = true, note = 'sid черты Magery в паке basic (конвенция репозитория)';

-- Типы повреждений (B61-62, множители раны B379)
UPSERT ref_damage_type:burn SET code = 'burn', wound_mult = 1.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 61 }], verified = true;
UPSERT ref_damage_type:cor SET code = 'cor', wound_mult = 1.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 61 }], verified = true;
UPSERT ref_damage_type:cr SET code = 'cr', wound_mult = 1.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 61 }], verified = true;
UPSERT ref_damage_type:cut SET code = 'cut', wound_mult = 1.5, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 61 }], verified = true;
UPSERT ref_damage_type:fat SET code = 'fat', wound_mult = 1.0, drains = 'FP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 61 }], verified = true;
UPSERT ref_damage_type:imp SET code = 'imp', wound_mult = 2.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 61 }], verified = true;
UPSERT ref_damage_type:⟨pi-⟩ SET code = 'pi-', wound_mult = 0.5, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 62 }], verified = true;
UPSERT ref_damage_type:pi SET code = 'pi', wound_mult = 1.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 62 }], verified = true;
UPSERT ref_damage_type:⟨pi+⟩ SET code = 'pi+', wound_mult = 1.5, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 62 }], verified = true;
UPSERT ref_damage_type:⟨pi++⟩ SET code = 'pi++', wound_mult = 2.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 62 }], verified = true;
UPSERT ref_damage_type:tox SET code = 'tox', wound_mult = 1.0, drains = 'HP', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 62 }], verified = true;

-- Модификаторы урона (B104-105; обозначения в таблицах оружия B269)
UPSERT ref_damage_modifier:dkb SET code = 'dkb', value_kind = 'none', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 104 }], verified = true;
UPSERT ref_damage_modifier:ex SET code = 'ex', value_kind = 'none', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 269 }], verified = true, note = 'в B104 улучшение обозначено (exp)';
UPSERT ref_damage_modifier:frag SET code = 'frag', value_kind = 'dice', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 104 }], verified = true, note = 'значение = кубики осколков';
UPSERT ref_damage_modifier:inc SET code = 'inc', value_kind = 'none', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 105 }], verified = true;
UPSERT ref_damage_modifier:rad SET code = 'rad', value_kind = 'none', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 105 }], verified = true;
UPSERT ref_damage_modifier:sur SET code = 'sur', value_kind = 'none', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 105 }], verified = true;

-- Зоны попадания (B552). Броски 3d6 по планам тела — 06_body_plans.
UPSERT ref_hit_location:eye SET code = 'eye', to_hit_mod = -9, paired = true, extra_dr = 0, cripple_divisor = 10.0, knockdown_mod = -10, knockdown_damage_types = [ref_damage_type:burn, ref_damage_type:cor, ref_damage_type:cr, ref_damage_type:cut, ref_damage_type:imp, ref_damage_type:⟨pi-⟩, ref_damage_type:pi, ref_damage_type:⟨pi+⟩, ref_damage_type:⟨pi++⟩], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi-⟩, mult: 4.0 }, { damage_type: ref_damage_type:pi, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 4.0 }, { damage_type: ref_damage_type:burn, mult: 4.0, condition: ref_wound_condition:narrow_beam }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'только imp/pi*/узкий луч burn; иначе как skull без доп. DR; tox без эффекта';
UPSERT ref_hit_location:skull SET code = 'skull', to_hit_mod = -7, paired = false, extra_dr = 2, cripple_divisor = NONE, knockdown_mod = -10, knockdown_damage_types = [ref_damage_type:burn, ref_damage_type:cor, ref_damage_type:cr, ref_damage_type:cut, ref_damage_type:imp, ref_damage_type:⟨pi-⟩, ref_damage_type:pi, ref_damage_type:⟨pi+⟩, ref_damage_type:⟨pi++⟩], wound_mults = [{ damage_type: ref_damage_type:cor, mult: 4.0 }, { damage_type: ref_damage_type:cr, mult: 4.0 }, { damage_type: ref_damage_type:cut, mult: 4.0 }, { damage_type: ref_damage_type:imp, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi-⟩, mult: 4.0 }, { damage_type: ref_damage_type:pi, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 4.0 }, { damage_type: ref_damage_type:burn, mult: 4.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'эффекты не действуют на tox; fat игнорирует зоны (B398)';
UPSERT ref_hit_location:face SET code = 'face', to_hit_mod = -5, paired = false, extra_dr = 0, cripple_divisor = NONE, knockdown_mod = -5, knockdown_damage_types = [ref_damage_type:burn, ref_damage_type:cor, ref_damage_type:cr, ref_damage_type:cut, ref_damage_type:imp, ref_damage_type:⟨pi-⟩, ref_damage_type:pi, ref_damage_type:⟨pi+⟩, ref_damage_type:⟨pi++⟩], wound_mults = [{ damage_type: ref_damage_type:cor, mult: 1.5 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'cor: крупная рана ослепляет глаз';
UPSERT ref_hit_location:neck SET code = 'neck', to_hit_mod = -5, paired = false, extra_dr = 0, cripple_divisor = NONE, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:cr, mult: 1.5 }, { damage_type: ref_damage_type:cor, mult: 1.5 }, { damage_type: ref_damage_type:cut, mult: 2.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true;
UPSERT ref_hit_location:groin SET code = 'groin', to_hit_mod = -3, paired = false, extra_dr = 0, cripple_divisor = NONE, knockdown_mod = -5, knockdown_damage_types = [ref_damage_type:cr], wound_mults = [], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'мужчины: двойной шок от cr (макс. -8) и -5 к нокдауну';
UPSERT ref_hit_location:torso SET code = 'torso', to_hit_mod = 0, paired = false, extra_dr = 0, cripple_divisor = NONE, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 398 }, { book: ref_book:B, page: 552 }], verified = true;
UPSERT ref_hit_location:vitals SET code = 'vitals', to_hit_mod = -3, paired = false, extra_dr = 0, cripple_divisor = NONE, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 3.0 }, { damage_type: ref_damage_type:⟨pi-⟩, mult: 3.0 }, { damage_type: ref_damage_type:pi, mult: 3.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 3.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 3.0 }, { damage_type: ref_damage_type:burn, mult: 2.0, condition: ref_wound_condition:narrow_beam }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'прочие атаки не могут целить в vitals';
UPSERT ref_hit_location:arm SET code = 'arm', to_hit_mod = -2, paired = true, extra_dr = 0, cripple_divisor = 2.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'со щитом -4; правая/левая — по плану тела';
UPSERT ref_hit_location:leg SET code = 'leg', to_hit_mod = -2, paired = true, extra_dr = 0, cripple_divisor = 2.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true;
UPSERT ref_hit_location:hand SET code = 'hand', to_hit_mod = -4, paired = true, extra_dr = 0, cripple_divisor = 3.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'со щитом -8; сторона 1d: 1-3 правая';
UPSERT ref_hit_location:foot SET code = 'foot', to_hit_mod = -4, paired = true, extra_dr = 0, cripple_divisor = 3.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }, { book: ref_book:B, page: 552 }], verified = true, note = 'сторона 1d: 1-3 правая';
UPSERT ref_hit_location:foreleg SET code = 'foreleg', to_hit_mod = -2, paired = true, extra_dr = 0, cripple_divisor = 2.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }, { book: ref_book:B, page: 552 }], verified = true, note = 'как leg';
UPSERT ref_hit_location:hind_leg SET code = 'hind_leg', to_hit_mod = -2, paired = true, extra_dr = 0, cripple_divisor = 2.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }, { book: ref_book:B, page: 552 }], verified = true, note = 'как leg';
UPSERT ref_hit_location:mid_leg SET code = 'mid_leg', to_hit_mod = -2, paired = true, extra_dr = 0, cripple_divisor = 2.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }, { book: ref_book:B, page: 552 }], verified = true, note = 'как leg (шестиногие)';
UPSERT ref_hit_location:wing SET code = 'wing', to_hit_mod = -2, paired = true, extra_dr = 0, cripple_divisor = 2.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }, { book: ref_book:B, page: 552 }], verified = true, note = 'как конечность для калечения; искалеченное крыло — нет полёта';
UPSERT ref_hit_location:tail SET code = 'tail', to_hit_mod = -3, paired = false, extra_dr = 0, cripple_divisor = NONE, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }, { book: ref_book:B, page: 552 }], verified = true, note = 'конечность (2) если Extra Arm/Striker/рыбий хвост, иначе как кисть/ступня (3) — зависит от существа';
UPSERT ref_hit_location:extremity SET code = 'extremity', to_hit_mod = -4, paired = true, extra_dr = 0, cripple_divisor = 3.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 553 }, { book: ref_book:B, page: 552 }], verified = true, note = 'кентавр: кисть/передняя/задняя ступня по 1d';
UPSERT ref_hit_location:brain SET code = 'brain', to_hit_mod = -7, paired = false, extra_dr = 1, cripple_divisor = NONE, knockdown_mod = -10, knockdown_damage_types = [ref_damage_type:burn, ref_damage_type:cor, ref_damage_type:cr, ref_damage_type:cut, ref_damage_type:imp, ref_damage_type:⟨pi-⟩, ref_damage_type:pi, ref_damage_type:⟨pi+⟩, ref_damage_type:⟨pi++⟩], wound_mults = [{ damage_type: ref_damage_type:cor, mult: 4.0 }, { damage_type: ref_damage_type:cr, mult: 4.0 }, { damage_type: ref_damage_type:cut, mult: 4.0 }, { damage_type: ref_damage_type:imp, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi-⟩, mult: 4.0 }, { damage_type: ref_damage_type:pi, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 4.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 4.0 }, { damage_type: ref_damage_type:burn, mult: 4.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 554 }, { book: ref_book:B, page: 552 }], verified = true, note = 'как skull, но DR 1';
UPSERT ref_hit_location:fin SET code = 'fin', to_hit_mod = -4, paired = true, extra_dr = 0, cripple_divisor = 3.0, knockdown_mod = NONE, knockdown_damage_types = [], wound_mults = [{ damage_type: ref_damage_type:imp, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi+⟩, mult: 1.0 }, { damage_type: ref_damage_type:⟨pi++⟩, mult: 1.0 }], pack = pack:basic, source_refs = [{ book: ref_book:B, page: 554 }, { book: ref_book:B, page: 552 }], verified = true, note = 'как кисть/ступня для калечения; -3 DX';

-- Класс легальности (B267)
UPSERT ref_lc:LC0 SET code = 'LC0', level = 0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 267 }], verified = true;
UPSERT ref_lc:LC1 SET code = 'LC1', level = 1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 267 }], verified = true;
UPSERT ref_lc:LC2 SET code = 'LC2', level = 2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 267 }], verified = true;
UPSERT ref_lc:LC3 SET code = 'LC3', level = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 267 }], verified = true;
UPSERT ref_lc:LC4 SET code = 'LC4', level = 4, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 267 }], verified = true;

-- Технологические уровни (B27)
UPSERT ref_tl:TL0 SET code = 'TL0', level = 0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL1 SET code = 'TL1', level = 1, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL2 SET code = 'TL2', level = 2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL3 SET code = 'TL3', level = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL4 SET code = 'TL4', level = 4, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL5 SET code = 'TL5', level = 5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL6 SET code = 'TL6', level = 6, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL7 SET code = 'TL7', level = 7, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL8 SET code = 'TL8', level = 8, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL9 SET code = 'TL9', level = 9, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL10 SET code = 'TL10', level = 10, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL11 SET code = 'TL11', level = 11, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true;
UPSERT ref_tl:TL12 SET code = 'TL12', level = 12, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 27 }], verified = true, note = 'TL12+';

-- Категории навыков (GURPS Skill Categories, SC)
UPSERT ref_skill_category:animal SET code = 'animal', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 1 }], verified = true;
UPSERT ref_skill_category:arts_entertainment SET code = 'arts_entertainment', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 1 }], verified = true;
UPSERT ref_skill_category:athletic SET code = 'athletic', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 1 }], verified = true;
UPSERT ref_skill_category:business SET code = 'business', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 2 }], verified = true;
UPSERT ref_skill_category:combat_weapon SET code = 'combat_weapon', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 2 }], verified = true;
UPSERT ref_skill_category:craft SET code = 'craft', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 2 }], verified = true;
UPSERT ref_skill_category:criminal_street SET code = 'criminal_street', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 2 }], verified = true;
UPSERT ref_skill_category:design_invention SET code = 'design_invention', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 3 }], verified = true;
UPSERT ref_skill_category:esoteric SET code = 'esoteric', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 3 }], verified = true;
UPSERT ref_skill_category:everyman SET code = 'everyman', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 3 }], verified = true;
UPSERT ref_skill_category:knowledge SET code = 'knowledge', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 3 }], verified = true;
UPSERT ref_skill_category:medical SET code = 'medical', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 4 }], verified = true;
UPSERT ref_skill_category:natural_sciences SET code = 'natural_sciences', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 4 }], verified = true;
UPSERT ref_skill_category:military SET code = 'military', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 4 }], verified = true;
UPSERT ref_skill_category:occult_magical SET code = 'occult_magical', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 4 }], verified = true;
UPSERT ref_skill_category:outdoor_exploration SET code = 'outdoor_exploration', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 4 }], verified = true;
UPSERT ref_skill_category:plant SET code = 'plant', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 5 }], verified = true;
UPSERT ref_skill_category:police SET code = 'police', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 5 }], verified = true;
UPSERT ref_skill_category:repair_maintenance SET code = 'repair_maintenance', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 5 }], verified = true;
UPSERT ref_skill_category:scholarly SET code = 'scholarly', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 5 }], verified = true;
UPSERT ref_skill_category:social SET code = 'social', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 5 }], verified = true;
UPSERT ref_skill_category:social_sciences_humanities SET code = 'social_sciences_humanities', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 6 }], verified = true;
UPSERT ref_skill_category:spy SET code = 'spy', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 6 }], verified = true;
UPSERT ref_skill_category:technical SET code = 'technical', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 6 }], verified = true;
UPSERT ref_skill_category:vehicle SET code = 'vehicle', pack = pack:basic, source_refs = [{ book: ref_book:SC, page: 7 }], verified = true;
UPSERT ref_skill_category:melee SET code = 'melee', pack = pack:basic, source_refs = [{ book: ref_book:B }], verified = false, note = 'ЗАГЛУШКА: используется сидами/тестами; в SC нет такой категории (ближайшая combat_weapon)';

-- Категории черт (B32-33)
UPSERT ref_trait_category:mental SET code = 'mental', axis = 'nature', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 32 }], verified = true;
UPSERT ref_trait_category:physical SET code = 'physical', axis = 'nature', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 32 }], verified = true;
UPSERT ref_trait_category:social SET code = 'social', axis = 'nature', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 32 }], verified = true;
UPSERT ref_trait_category:exotic SET code = 'exotic', axis = 'origin', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 32 }], verified = true;
UPSERT ref_trait_category:supernatural SET code = 'supernatural', axis = 'origin', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 33 }], verified = true;
UPSERT ref_trait_category:mundane SET code = 'mundane', axis = 'origin', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 33 }], verified = true;

-- Самоконтроль (B121)
UPSERT ref_self_control:cr6 SET code = 'cr6', kind = 'roll', roll = 6, cost_mult = 2.0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 121 }], verified = true;
UPSERT ref_self_control:cr9 SET code = 'cr9', kind = 'roll', roll = 9, cost_mult = 1.5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 121 }], verified = true;
UPSERT ref_self_control:cr12 SET code = 'cr12', kind = 'roll', roll = 12, cost_mult = 1.0, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 121 }], verified = true;
UPSERT ref_self_control:cr15 SET code = 'cr15', kind = 'roll', roll = 15, cost_mult = 0.5, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 121 }], verified = true;

-- Оружейные сноски — вне задачи, оставлено заглушкой
UPSERT ref_weapon_note:dagger SET code = 'dagger', symbol = '†', column = 'st', pack = pack:basic;

-- Магия: школы (M, главы 3-26; страница начала главы по оглавлению)
UPSERT ref_college:air SET code = 'air', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 23 }], verified = true;
UPSERT ref_college:animal SET code = 'animal', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 29 }], verified = true;
UPSERT ref_college:body_control SET code = 'body_control', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 35 }], verified = true;
UPSERT ref_college:communication_empathy SET code = 'communication_empathy', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 44 }], verified = true;
UPSERT ref_college:earth SET code = 'earth', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 50 }], verified = true;
UPSERT ref_college:enchantment SET code = 'enchantment', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 56 }], verified = true;
UPSERT ref_college:fire SET code = 'fire', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 72 }], verified = true;
UPSERT ref_college:food SET code = 'food', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 77 }], verified = true;
UPSERT ref_college:gate SET code = 'gate', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 80 }], verified = true;
UPSERT ref_college:healing SET code = 'healing', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 88 }], verified = true;
UPSERT ref_college:illusion_creation SET code = 'illusion_creation', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 95 }], verified = true;
UPSERT ref_college:knowledge SET code = 'knowledge', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 100 }], verified = true;
UPSERT ref_college:light_darkness SET code = 'light_darkness', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 110 }], verified = true;
UPSERT ref_college:making_breaking SET code = 'making_breaking', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 115 }], verified = true;
UPSERT ref_college:meta SET code = 'meta', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 121 }], verified = true;
UPSERT ref_college:mind_control SET code = 'mind_control', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 133 }], verified = true;
UPSERT ref_college:movement SET code = 'movement', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 142 }], verified = true;
UPSERT ref_college:necromantic SET code = 'necromantic', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 149 }], verified = true;
UPSERT ref_college:plant SET code = 'plant', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 161 }], verified = true;
UPSERT ref_college:protection_warning SET code = 'protection_warning', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 166 }], verified = true;
UPSERT ref_college:sound SET code = 'sound', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 171 }], verified = true;
UPSERT ref_college:technological SET code = 'technological', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 175 }], verified = true;
UPSERT ref_college:water SET code = 'water', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 184 }], verified = true;
UPSERT ref_college:weather SET code = 'weather', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 193 }], verified = true;
UPSERT ref_college:college_a SET code = 'college_a', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 11 }], verified = false, note = 'ЗАГЛУШКА: используется сидами/тестами';
UPSERT ref_college:college_b SET code = 'college_b', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 11 }], verified = false, note = 'ЗАГЛУШКА: используется сидами/тестами';

-- Классы заклинаний (M11-14)
UPSERT ref_spell_class:regular SET code = 'regular', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 11 }], verified = true;
UPSERT ref_spell_class:area SET code = 'area', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 11 }], verified = true;
UPSERT ref_spell_class:melee SET code = 'melee', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 11 }], verified = true;
UPSERT ref_spell_class:missile SET code = 'missile', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 12 }], verified = true;
UPSERT ref_spell_class:blocking SET code = 'blocking', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 12 }], verified = true;
UPSERT ref_spell_class:information SET code = 'information', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 13 }], verified = true;
UPSERT ref_spell_class:enchantment SET code = 'enchantment', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 14 }], verified = true;
UPSERT ref_spell_class:special SET code = 'special', role = 'main', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 14 }], verified = true;
UPSERT ref_spell_class:resisted SET code = 'resisted', role = 'modifier', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 13 }], verified = true;
UPSERT ref_spell_system:standard SET code = 'standard', pack = pack:basic, source_refs = [{ book: ref_book:M, page: 6 }], verified = false, note = 'вне задачи; системная запись';

-- Уровни маны (M6)
UPSERT ref_mana_level:none SET code = 'none', rank = 0, pack = pack:basic, source_refs = [{ book: ref_book:M, page: 6 }], verified = true;
UPSERT ref_mana_level:low SET code = 'low', rank = 1, pack = pack:basic, source_refs = [{ book: ref_book:M, page: 6 }], verified = true;
UPSERT ref_mana_level:normal SET code = 'normal', rank = 2, pack = pack:basic, source_refs = [{ book: ref_book:M, page: 6 }], verified = true;
UPSERT ref_mana_level:high SET code = 'high', rank = 3, pack = pack:basic, source_refs = [{ book: ref_book:M, page: 6 }], verified = true;
UPSERT ref_mana_level:very_high SET code = 'very_high', rank = 4, pack = pack:basic, source_refs = [{ book: ref_book:M, page: 6 }], verified = true;
UPDATE pack:basic SET spell_system = ref_spell_system:standard;
