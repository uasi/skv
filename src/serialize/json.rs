use parse;
use serde::json;
use std::collections::btree_map::BTreeMap;
use std::fs::File;
use std::io::Read;

impl parse::Value {
    fn to_json(&self) -> json::Value {
        let maybe_s = match *self {
            parse::Value::DataFile(ref path)    => slurp(path),
            parse::Value::DataString(ref s)     => Some(s.to_string()),
            parse::Value::FormFile(ref path)    => slurp(path),
            parse::Value::HttpHeader(ref s)     => Some(s.to_string()),
            parse::Value::RawJsonString(ref s)  => Some(s.to_string()),
            parse::Value::RawJsonFile(ref path) => slurp(path).and_then(|s| json::from_str(&s).ok()),
            parse::Value::UrlParam(ref s)       => Some(s.to_string()),
        };
        match maybe_s {
            Some(s) => json::Value::String(s),
            None    => json::Value::Null,
        }
    }
}

pub fn to_json(pairs: &[parse::Pair]) -> String {
    let mut map = BTreeMap::new();
    for pair in pairs.iter() {
        let k = pair.0.clone();
        let v = pair.1.clone().map(|v| v.to_json()).unwrap_or(json::Value::Null);
        map.insert(k, v);
    }
    let root = json::Value::Object(map);
    json::ser::to_string(&root).unwrap_or("".to_string())
}

fn slurp(path: &str) -> Option<String> {
    File::open(path).ok().as_mut().and_then(|f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| Some(s))
    })
}
