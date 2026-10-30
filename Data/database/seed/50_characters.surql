-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
UPSERT character:test_hero SET name = 'Тестовый герой', player = 'Undeadguy', campaign = 'test', tl = ref_tl:TL3,
	mana_level = ref_mana_level:normal,
	attributes = [{ attribute: ref_attribute:ST, levels: 1 }, { attribute: ref_attribute:DX, levels: 2 }, { attribute: ref_attribute:Speed, levels: 0.25 }];
-- Личный пак персонажа для собственных записей
UPSERT pack:custom_test_hero SET name = 'Custom: test_hero', abbrev = 'C-test_hero', kind = 'custom', priority = 1000, owner = character:test_hero;
UPSERT trait:['custom_test_hero', 'oath_a'] SET kind = 'quirk', cost = { type: 'fixed', base: -1 };

DELETE has_skill WHERE in = character:test_hero; DELETE has_trait WHERE in = character:test_hero;
DELETE has_technique WHERE in = character:test_hero; DELETE has_spell WHERE in = character:test_hero;
DELETE owns_item WHERE in = character:test_hero; DELETE point_ledger WHERE character = character:test_hero;

RELATE character:test_hero->has_skill->skill:['basic', 'sword_a'] SET points = 4;
RELATE character:test_hero->has_skill->skill:['basic', 'outdoor_a'] SET points = 2, spec = 'spec_a';
RELATE character:test_hero->has_skill->skill:['basic', 'unarmed_a'] SET points = 1;
RELATE character:test_hero->has_technique->technique:['basic', 'strike_a'] SET points = 1;
RELATE character:test_hero->has_trait->trait:['basic', 'magery'] SET level = 1, modifiers = [{ modifier: modifier:['basic', 'limit_a'] }];
RELATE character:test_hero->has_trait->trait:['basic', 'claws_a'];
RELATE character:test_hero->has_trait->trait:['custom_test_hero', 'oath_a'];
RELATE character:test_hero->has_spell->spell:['basic', 'spell_a'] SET points = 1;
RELATE character:test_hero->owns_item:hero_bag->item:['basic', 'bag_a'] SET location = 'back';
RELATE character:test_hero->owns_item:hero_sword->item:['basic', 'sword_item_a'] SET equipped = true, location = 'belt';
RELATE character:test_hero->owns_item:hero_knuckles->item:['basic', 'knuckles_a'] SET contained_in = owns_item:hero_bag;

INSERT INTO point_ledger [
	{ character: character:test_hero, kind: 'award', delta: 100, note: 'стартовые очки' },
	{ character: character:test_hero, kind: 'spend', delta: -20, target: ref_attribute:DX },
	{ character: character:test_hero, kind: 'spend', delta: -4, target: skill:['basic', 'sword_a'] },
	{ character: character:test_hero, kind: 'award', delta: 2, session: 's01' }
];
