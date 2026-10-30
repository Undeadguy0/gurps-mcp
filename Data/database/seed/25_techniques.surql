-- ЗАГЛУШКИ (placeholder): не данные GURPS. Все записи verified = false; наполнение — отдельная задача (Мастер).
-- Приём, производный от безоружного шаблона (стиль MA)
UPSERT technique:['basic', 'strike_a'] SET parent_skill = skill:['basic', 'unarmed_a'], difficulty = ref_difficulty:A,
	default_mod = -2, max_mod = 0, derived_from = attack_mode:['basic', 'punch'], attack_delta = { damage_mod: 1 },
	prereqs = { type: 'skill', skill: skill:['basic', 'unarmed_a'], min: 10 };
UPDATE attack_mode:['basic', 'kick'] SET technique = technique:['basic', 'strike_a'];
