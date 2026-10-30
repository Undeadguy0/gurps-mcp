-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
DELETE defaults_to; DELETE attr_default;
INSERT INTO attr_default [
	{ id: 'sword_a_dx', skill: skill:['basic', 'sword_a'], attribute: ref_attribute:DX, mod: -5 },
	{ id: 'outdoor_a_per', skill: skill:['basic', 'outdoor_a'], attribute: ref_attribute:Per, mod: -5 }
];
RELATE skill:['basic', 'sword_a']->defaults_to->skill:['basic', 'sword_b'] SET mod = -2;
RELATE skill:['basic', 'sword_b']->defaults_to->skill:['basic', 'sword_a'] SET mod = -2;
RELATE skill:['basic', 'outdoor_a']->defaults_to->skill:['basic', 'outdoor_a'] SET mod = -3, from_spec = 'spec_a', spec = 'spec_b';
