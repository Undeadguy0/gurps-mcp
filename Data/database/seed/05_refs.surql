-- Справочники: ТОЛЬКО минимум, нужный остальным сидам.
-- placeholder, to be filled by Мастер. Все строки verified = false,
-- названия/числа не сверены с книгами.
UPSERT ref_attribute:ST SET code = 'ST', name_ru = 'Сила', name_en = 'Strength', kind = 'basic', pack = pack:basic, verified = false;
UPSERT ref_attribute:DX SET code = 'DX', name_ru = 'Ловкость', name_en = 'Dexterity', kind = 'basic', pack = pack:basic, verified = false;
UPSERT ref_attribute:IQ SET code = 'IQ', name_ru = 'Интеллект', name_en = 'Intelligence', kind = 'basic', pack = pack:basic, verified = false;
UPSERT ref_attribute:Per SET code = 'Per', name_ru = 'Восприятие', name_en = 'Perception', kind = 'secondary', pack = pack:basic, verified = false;

UPSERT ref_difficulty:E SET code = 'E', name_ru = 'Лёгкий', name_en = 'Easy', pack = pack:basic, verified = false;
UPSERT ref_difficulty:A SET code = 'A', name_ru = 'Средний', name_en = 'Average', for_techniques = true, pack = pack:basic, verified = false;
UPSERT ref_difficulty:H SET code = 'H', name_ru = 'Сложный', name_en = 'Hard', for_techniques = true, pack = pack:basic, verified = false;

UPSERT ref_skill_category:melee SET code = 'melee', name_ru = 'Ближний бой', pack = pack:basic, verified = false;

UPSERT ref_trait_category:mental SET code = 'mental', name_ru = 'Ментальная', axis = 'nature', pack = pack:basic, verified = false;
UPSERT ref_trait_category:supernatural SET code = 'supernatural', name_ru = 'Сверхъестественная', axis = 'origin', pack = pack:basic, verified = false;
UPSERT ref_self_control:cr12 SET code = 'cr12', name_ru = 'Сопротивляется довольно часто (12)', roll = 12, cost_mult = 1.0, pack = pack:basic, verified = false;

UPSERT ref_damage_type:cut SET code = 'cut', name_ru = 'Режущий', name_en = 'cutting', pack = pack:basic, verified = false;
UPSERT ref_damage_type:cr SET code = 'cr', name_ru = 'Дробящий', name_en = 'crushing', pack = pack:basic, verified = false;
UPSERT ref_damage_type:imp SET code = 'imp', name_ru = 'Колющий', name_en = 'impaling', pack = pack:basic, verified = false;

UPSERT ref_hit_location:torso SET code = 'torso', name_ru = 'Торс', name_en = 'Torso', pack = pack:basic, verified = false;
UPSERT ref_weapon_note:dagger SET code = 'dagger', symbol = '†', column = 'st', name_ru = 'Сноска † к ST', pack = pack:basic, verified = false;

UPSERT ref_tl:TL0 SET code = 'TL0', level = 0, name_ru = 'TL0', pack = pack:basic, verified = false;
UPSERT ref_tl:TL2 SET code = 'TL2', level = 2, name_ru = 'TL2', pack = pack:basic, verified = false;
