-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
-- Переводы (name / description / spec.<sid>). Пересоздаются целиком.
DELETE translation;
FOR $r IN [
	[ref_attribute:ST, 'ST'], [ref_attribute:DX, 'DX'], [ref_attribute:IQ, 'IQ'], [ref_attribute:HT, 'HT'],
	[ref_damage_type:cut, 'cut'], [ref_damage_type:cr, 'cr'], [ref_damage_type:imp, 'imp'],
	[skill:['basic', 'sword_a'], 'Sword A'], [skill:['homebrew', 'sword_a'], 'Sword A (HB)'], [skill:['basic', 'sword_b'], 'Sword B'],
	[skill:['basic', 'unarmed_a'], 'Unarmed A'], [skill:['basic', 'outdoor_a'], 'Outdoor A'], [skill:['basic', 'lore_a'], 'Lore A'],
	[attack_mode:['basic', 'punch'], 'Punch'], [attack_mode:['basic', 'kick'], 'Kick'], [attack_mode:['basic', 'bite'], 'Bite'],
	[technique:['basic', 'strike_a'], 'Strike A'], [trait:['basic', 'magery'], 'Magery'], [trait:['basic', 'claws_a'], 'Claws A'],
	[spell:['basic', 'spell_a'], 'Spell A'], [spell:['basic', 'spell_b'], 'Spell B'],
	[item:['basic', 'sword_item_a'], 'Sword item A'], [item:['basic', 'shield_a'], 'Shield A'], [item:['basic', 'bag_a'], 'Bag A']
] {
	CREATE translation SET record = $r[0], lang = 'en', field = 'name', text = $r[1];
	CREATE translation SET record = $r[0], lang = 'ru', field = 'name', text = 'ЗАГЛУШКА ' + $r[1];
};
CREATE translation SET record = skill:['basic', 'sword_a'], lang = 'ru', field = 'description', text = 'ЗАГЛУШКА: описание навыка';
CREATE translation SET record = skill:['basic', 'outdoor_a'], lang = 'ru', field = 'spec.spec_a', text = 'ЗАГЛУШКА специализация A';
CREATE translation SET record = skill:['basic', 'outdoor_a'], lang = 'ru', field = 'spec.spec_b', text = 'ЗАГЛУШКА специализация B';
CREATE translation SET record = trait:['custom_test_hero', 'oath_a'], lang = 'ru', field = 'name', text = 'Личная клятва (тест)';
CREATE translation SET record = skill:['basic', 'outdoor_a'], lang = 'en', field = 'description', text = 'PLACEHOLDER: requires specializations';
