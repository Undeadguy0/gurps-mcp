-- Связи черт. Пререквизиты — деревом в поле prereqs; бонусы талантов — features (skill_bonus).
-- Взаимоисключение черт.
DEFINE TABLE OVERWRITE excludes TYPE RELATION IN trait OUT trait SCHEMAFULL;
-- Альтернативные черты.
DEFINE TABLE OVERWRITE alternative_of TYPE RELATION IN trait OUT trait SCHEMAFULL;
