use parse;
use serde::json;
use std::collections::btree_map::BTreeMap;
use std::fs::File;
use std::io::{self, Read};

impl parse::Value {
    fn to_json(&self) -> Option<json::Value> {
        match *self {
            parse::Value::DataFile(ref path) => {
                slurp(path)
                    .ok()
                    .and_then(|s| Some(json::Value::String(s)))
            }
            parse::Value::DataString(ref s) => {
                Some(json::Value::String(s.to_string()))
            }
            parse::Value::FormFile(ref path) => {
                slurp(path)
                    .ok()
                    .and_then(|s| Some(json::Value::String(s)))
            }
            parse::Value::HttpHeader(ref s) => {
                Some(json::Value::String(s.to_string()))
            }
            parse::Value::RawJsonString(ref s) => {
                json::from_str(s).unwrap_or(None)
            }
            parse::Value::RawJsonFile(ref path) => {
                slurp(path)
                    .ok()
                    .and_then(|s| json::from_str(&s).ok())
            }
            parse::Value::UrlParam(ref s) => {
                Some(json::Value::String(s.to_string()))
            }
        }
    }
}

pub fn to_json(pairs: &[parse::Pair]) -> String {
    let mut map = BTreeMap::new();
    for pair in pairs.iter() {
        let maybe_v = pair.1.clone().and_then(|v| v.to_json());
        if let Some(v) = maybe_v {
            let k = pair.0.clone();
            map.insert(k, v);
        }
    }
    let root = json::Value::Object(map);
    json::ser::to_string(&root).unwrap_or("".to_string())
}

fn slurp(path: &str) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}
