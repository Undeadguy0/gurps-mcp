-- Анализаторы полнотекстового поиска по translation.text_en / translation.text_ru (по индексу на язык).
DEFINE ANALYZER OVERWRITE gurps_en TOKENIZERS class FILTERS lowercase, ascii, snowball(english);
DEFINE ANALYZER OVERWRITE gurps_ru TOKENIZERS class FILTERS lowercase, snowball(russian);
