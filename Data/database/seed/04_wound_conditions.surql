-- Условия особых множителей раны (ref_hit_location.wound_mults[*].condition).
-- narrow_beam перенесён из заметки Мастера у vitals («burn x2 только узкий луч»); verified = false — сверить.
UPSERT ref_wound_condition:narrow_beam SET code = 'narrow_beam', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }], verified = false,
	note = 'ожог узким лучом (лазер и т. п.); страница — сверить';
