//! Модульные проверки без сервера: id, разбор урона/досягаемости, Fluent, Rec.
use sheet_bridge::catalog::damage::{parse_damage, parse_reach};
use sheet_bridge::ids;
use sheet_bridge::sheet::ftl::{parse_ftl, write_ftl, LocEntry};
use sheet_bridge::sql::Rec;

#[test]
fn kebab_sid() {
    assert_eq!(ids::kebab_to_sid("first-aid").unwrap(), "first_aid");
    assert_eq!(ids::sid_to_kebab("first_aid"), "first-aid");
    assert!(ids::kebab_to_sid("Bad Id").is_err());
    assert_eq!(ids::slug("Light and Darkness"), "light_and_darkness");
    assert_eq!(ids::stat_to_ref("BasicSpeed"), "Speed");
    assert_eq!(ids::ref_to_stat("Move"), "BasicMove");
    assert_eq!(ids::parse_page("B271"), Some(("B".into(), 271)));
    assert_eq!(ids::parse_page("example"), None);
}

#[test]
fn damage_and_reach() {
    assert_eq!(parse_damage("sw+1"), Some((Some("sw"), None, 1)));
    assert_eq!(parse_damage("thr-1"), Some((Some("thr"), None, -1)));
    assert_eq!(parse_damage("2d-1"), Some((None, Some(2), -1)));
    assert_eq!(parse_damage("bogus"), None);
    assert_eq!(parse_reach("C,1"), Some(vec![0, 1]));
    assert_eq!(parse_reach("1-3"), Some(vec![1, 2, 3]));
    assert_eq!(parse_reach("2*"), Some(vec![2]));
}

#[test]
fn ftl_escaping_roundtrip() {
    let e = LocEntry {
        name: Some("Щит {тест}".into()),
        description: Some("Строка 1\n\n  * пункт\n[x] .y\n".into()),
    };
    let src = write_ftl("заголовок", &[("skill-shield".into(), e.clone())]);
    let got = parse_ftl(&src, "t").unwrap();
    let g = &got["skill-shield"];
    assert_eq!(g.name, e.name);
    assert_eq!(
        g.description.as_deref().map(str::trim_end),
        e.description.as_deref().map(str::trim_end)
    );
}

#[test]
fn rec_parse_display() {
    let r = Rec::parse("skill:['basic_set_sample', 'first_aid']").unwrap();
    assert_eq!(r.pack_sid(), Some(("basic_set_sample", "first_aid")));
    assert_eq!(Rec::parse(&r.to_string()).unwrap(), r);
    let c = Rec::parse("ref_attribute:ST").unwrap();
    assert_eq!(c.code_str(), Some("ST"));
    let odd = Rec::code("ref_college", "light-and darkness");
    assert_eq!(Rec::parse(&odd.to_string()).unwrap(), odd);
}
