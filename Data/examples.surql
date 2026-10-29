-- Примеры запросов (проверочные). Запуск: surreal sql -e http://127.0.0.1:8000 -u root -p root --ns gurps --db gurps < examples.surql
SELECT count() FROM pack GROUP ALL; SELECT count() FROM skill GROUP ALL; SELECT count() FROM trait GROUP ALL; SELECT count() FROM item GROUP ALL; SELECT count() FROM defaults_to GROUP ALL; SELECT count() FROM has_skill GROUP ALL;
SELECT name, search::score(1) AS s FROM skill WHERE name_ru @1@ 'выживания' ORDER BY s DESC;
SELECT name FROM skill WHERE name @1@ 'swords';
SELECT name, ->defaults_to->skill.name AS defaults, <-defaults_to<-skill.name AS from FROM skill:broadsword;
SELECT name, ->defaults_to.mod AS mods, ->defaults_to->skill->defaults_to->skill.name AS two_hops FROM skill:shortsword;
fn::effective('skill', 'Broadsword');
SELECT name, _refs FROM item;
SELECT name, ->has_skill.{lvl: level, sk: out.name, spec: specialization} AS skills, ->has_trait->trait.name AS traits FROM character;
SELECT in.name AS hb, in.pack.name AS pk, out.pack.name AS orig FROM overrides;
