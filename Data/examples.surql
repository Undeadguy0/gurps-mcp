-- Примеры запросов (проверочные). Запуск: surreal sql -e http://127.0.0.1:8000 -u root -p root --ns gurps --db gurps < examples.surql
-- Количество записей
SELECT count() FROM skill GROUP ALL; SELECT count() FROM spell GROUP ALL; SELECT count() FROM translation GROUP ALL;
-- Поиск по переводам (полнотекст), затем запись
SELECT record, text, search::score(1) AS s FROM translation WHERE lang = 'ru' AND text @1@ 'заглушка специализация' ORDER BY s DESC LIMIT 5;
-- Название на нужном языке с откатом на en
RETURN fn::tr(skill:['basic', 'outdoor_a'], 'ru', 'name');
-- Действующая версия по sid (override: побеждает пак с большим priority)
RETURN fn::effective('skill', 'sword_a').id;
-- Все версии одного sid по пакам
SELECT id, pack.priority AS prio FROM skill WHERE sid = 'sword_a' ORDER BY prio DESC;
-- Дефолты
SELECT sid, ->defaults_to.{to: out.sid, mod: mod} AS defaults FROM skill:['basic', 'sword_a'];
-- Безоружные шаблоны и правила, которые их меняют (расчёт — в коде)
SELECT sid, damage, reach FROM attack_mode WHERE natural = true;
SELECT id, features FROM trait, item, skill WHERE features.any(|$f| $f.kind = 'damage_bonus');
-- Техники, производные от шаблона атаки
SELECT sid, derived_from, attack_delta FROM technique WHERE derived_from != NONE;
-- Заклинания школы и их дерево пререквизитов
SELECT sid, classes, difficulty, prereqs FROM spell WHERE ref_college:college_a IN colleges;
-- Все ссылки дерева пререквизитов
RETURN fn::prereq_refs((SELECT VALUE prereqs FROM ONLY spell:['basic', 'spell_b']));
-- Персонаж: купленные атрибуты, навыки (с выбранной специализацией), черты, заклинания
SELECT name, attributes, ->has_skill.{skill: out.sid, points: points, spec: spec} AS skills, ->has_trait.{trait: out.id, level: level} AS traits, ->has_spell.{spell: out.sid, points: points} AS spells FROM character:test_hero;
-- Инвентарь с вложенностью
SELECT id, out.sid AS item, contained_in, equipped FROM owns_item WHERE in = character:test_hero;
-- Журнал очков и итог
SELECT at, kind, delta, target, note FROM point_ledger WHERE character = character:test_hero ORDER BY at;
RETURN math::sum(SELECT VALUE delta FROM point_ledger WHERE character = character:test_hero);
-- Данные для генерации .ftl
SELECT record, field, text FROM translation WHERE lang = 'ru' ORDER BY record, field;
