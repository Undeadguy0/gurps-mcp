-- Тестовая фикстура: школы магии из образца gurps-sheet (заглушки).
UPSERT ref_college:fire SET code = 'fire', pack = pack:basic;
UPSERT ref_college:light_and_darkness SET code = 'light_and_darkness', pack = pack:basic;
UPSERT ref_college:movement SET code = 'movement', pack = pack:basic;
UPSERT ref_college:knowledge SET code = 'knowledge', pack = pack:basic;
CREATE translation CONTENT { record: ref_college:fire, lang: 'en', field: 'name', text: 'Fire' };
CREATE translation CONTENT { record: ref_college:light_and_darkness, lang: 'en', field: 'name', text: 'Light and Darkness' };
CREATE translation CONTENT { record: ref_college:movement, lang: 'en', field: 'name', text: 'Movement' };
CREATE translation CONTENT { record: ref_college:knowledge, lang: 'en', field: 'name', text: 'Knowledge' };
