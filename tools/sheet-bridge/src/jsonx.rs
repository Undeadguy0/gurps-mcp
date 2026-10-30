//! Чтение полей из JSON, полученного от SurrealDB.

use crate::sql::Rec;
use serde_json::Value as J;

pub fn s<'a>(v: &'a J, k: &str) -> Option<&'a str> {
    v.get(k)?.as_str()
}
pub fn i(v: &J, k: &str) -> Option<i64> {
    let x = v.get(k)?;
    x.as_i64().or_else(|| x.as_f64().filter(|f| f.fract() == 0.0).map(|f| f as i64))
}
pub fn f(v: &J, k: &str) -> Option<f64> {
    v.get(k)?.as_f64()
}
pub fn b(v: &J, k: &str) -> Option<bool> {
    v.get(k)?.as_bool()
}
pub fn rec(v: &J, k: &str) -> Option<Rec> {
    Rec::parse(v.get(k)?.as_str()?)
}
pub fn recs(v: &J, k: &str) -> Vec<Rec> {
    arr(v, k).iter().filter_map(|x| x.as_str().and_then(Rec::parse)).collect()
}
pub fn arr<'a>(v: &'a J, k: &str) -> &'a [J] {
    v.get(k).and_then(|x| x.as_array()).map(Vec::as_slice).unwrap_or(&[])
}
/// Код справочника (`ref_attribute:DX` -> `DX`).
pub fn code(v: &J, k: &str) -> Option<String> {
    rec(v, k).and_then(|r| r.code_str().map(str::to_string))
}
/// sid каталожной ссылки.
pub fn sid_of(v: &J, k: &str) -> Option<String> {
    rec(v, k).and_then(|r| r.pack_sid().map(|(_, s)| s.to_string()))
}
