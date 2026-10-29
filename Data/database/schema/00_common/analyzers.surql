-- Анализаторы полнотекстового поиска.
-- en: для name (английские названия), ru: для name_ru.
DEFINE ANALYZER OVERWRITE gurps_en TOKENIZERS class FILTERS lowercase, ascii, snowball(english);
DEFINE ANALYZER OVERWRITE gurps_ru TOKENIZERS class FILTERS lowercase, snowball(russian);
