-- ВНИМАНИЕ: данные набраны по памяти из GURPS Basic Set 4e и НЕ СВЕРЕНЫ с книгой.
-- Номера страниц не указаны (page = NONE) — проверить цифры и страницы по книгам.
DELETE defaults_to; DELETE attr_default; DELETE overrides;
INSERT INTO attr_default [
	{ id: 'broadsword_dx', skill: skill:broadsword, attribute: 'DX', mod: -5 },
	{ id: 'stealth_dx', skill: skill:stealth, attribute: 'DX', mod: -5 },
	{ id: 'stealth_iq', skill: skill:stealth, attribute: 'IQ', mod: -5 },
	{ id: 'survival_per', skill: skill:survival, attribute: 'Per', mod: -5 }
];
RELATE skill:broadsword->defaults_to->skill:shortsword SET mod = -2;
RELATE skill:broadsword->defaults_to->skill:two_handed_sword SET mod = -4;
RELATE skill:shortsword->defaults_to->skill:broadsword SET mod = -2;
RELATE skill:two_handed_sword->defaults_to->skill:broadsword SET mod = -4;
RELATE skill:survival->defaults_to->skill:naturalist SET mod = -3;
RELATE skill:survival_woodlands->defaults_to->skill:survival_mountain SET mod = -3;
RELATE skill:hb_broadsword->overrides->skill:broadsword SET reason = 'тест';
