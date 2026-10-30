-- Условия особых множителей раны (ref_hit_location.wound_mults[*].condition).
-- narrow_beam перенесён из заметки Мастера у vitals («burn x2 только узкий луч»); сверено с B399 (определение узкого луча).
UPSERT ref_wound_condition:narrow_beam SET code = 'narrow_beam', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 399 }], verified = true,
	note = 'burn дистанционной атакой, не струя/конус/площадь/взрыв/последующая';
