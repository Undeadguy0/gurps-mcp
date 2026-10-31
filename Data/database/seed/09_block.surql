-- Block (B375): Block = block_base + навык Shield/Cloak / block_skill_divisor, дроби отбрасываются. Сверено с PDF Basic Set (B375 = PDF 379).
UPSERT ref_rule:block_base SET code = 'block_base', value = 3, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 375 }], verified = true, note = 'Block = 3 + skill/2';
UPSERT ref_rule:block_skill_divisor SET code = 'block_skill_divisor', value = 2, pack = pack:basic, source_refs = [{ book: ref_book:B, page: 375 }], verified = true;
UPSERT ref_rule:block_rounding SET code = 'block_rounding', text = 'floor', pack = pack:basic, source_refs = [{ book: ref_book:B, page: 375 }], verified = true, note = 'дроби отбрасываются (пример в книге: Shield-11 даёт Block 8)';
