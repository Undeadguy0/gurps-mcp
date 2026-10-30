-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
-- Только минимум кодов, нужный остальным сидам. Названия — в 90_translations.
UPSERT ref_attribute:ST SET code = 'ST', kind = 'basic', pack = pack:basic;
UPSERT ref_attribute:DX SET code = 'DX', kind = 'basic', pack = pack:basic;
UPSERT ref_attribute:IQ SET code = 'IQ', kind = 'basic', pack = pack:basic;
UPSERT ref_attribute:HT SET code = 'HT', kind = 'basic', pack = pack:basic;
UPSERT ref_attribute:Will SET code = 'Will', kind = 'secondary', pack = pack:basic;
UPSERT ref_attribute:Per SET code = 'Per', kind = 'secondary', pack = pack:basic;
UPSERT ref_attribute:HP SET code = 'HP', kind = 'secondary', pack = pack:basic;
UPSERT ref_attribute:FP SET code = 'FP', kind = 'secondary', pack = pack:basic;
UPSERT ref_attribute:Speed SET code = 'Speed', kind = 'secondary', step = 0.25, pack = pack:basic;
UPSERT ref_attribute:Move SET code = 'Move', kind = 'secondary', pack = pack:basic;

UPSERT ref_difficulty:E SET code = 'E', pack = pack:basic;
UPSERT ref_difficulty:A SET code = 'A', for_techniques = true, pack = pack:basic;
UPSERT ref_difficulty:H SET code = 'H', for_techniques = true, pack = pack:basic;
UPSERT ref_difficulty:VH SET code = 'VH', pack = pack:basic;

UPSERT ref_skill_category:melee SET code = 'melee', pack = pack:basic;
UPSERT ref_trait_category:mental SET code = 'mental', axis = 'nature', pack = pack:basic;
UPSERT ref_trait_category:physical SET code = 'physical', axis = 'nature', pack = pack:basic;
UPSERT ref_self_control:cr12 SET code = 'cr12', roll = 12, cost_mult = 1.0, pack = pack:basic;

UPSERT ref_damage_type:cut SET code = 'cut', pack = pack:basic;
UPSERT ref_damage_type:cr SET code = 'cr', pack = pack:basic;
UPSERT ref_damage_type:imp SET code = 'imp', pack = pack:basic;
UPSERT ref_hit_location:torso SET code = 'torso', pack = pack:basic;
UPSERT ref_weapon_note:dagger SET code = 'dagger', symbol = '†', column = 'st', pack = pack:basic;
UPSERT ref_tl:TL0 SET code = 'TL0', level = 0, pack = pack:basic;
UPSERT ref_tl:TL2 SET code = 'TL2', level = 2, pack = pack:basic;
UPSERT ref_tl:TL3 SET code = 'TL3', level = 3, pack = pack:basic;

-- Магия
UPSERT ref_college:college_a SET code = 'college_a', pack = pack:basic;
UPSERT ref_college:college_b SET code = 'college_b', pack = pack:basic;
UPSERT ref_spell_class:regular SET code = 'regular', role = 'main', pack = pack:basic;
UPSERT ref_spell_class:resisted SET code = 'resisted', role = 'modifier', pack = pack:basic;
UPSERT ref_spell_system:standard SET code = 'standard', pack = pack:basic;
UPSERT ref_mana_level:normal SET code = 'normal', rank = 2, pack = pack:basic;
UPDATE pack:basic SET spell_system = ref_spell_system:standard;
