use parse;
use serde::json;
use std::collections::BTreeMap;

impl parse::Value {
    fn to_json_for_dump(&self) -> json::Value {
        let mut map = BTreeMap::new();
        let (ty, value) = match *self {
            parse::Value::DataFile(ref path)    => ("data_file", path),
            parse::Value::DataString(ref s)     => ("data_string", s),
            parse::Value::FormFile(ref path)    => ("form_file", path),
            parse::Value::HttpHeader(ref s)     => ("http_header", s),
            parse::Value::RawJsonString(ref s)  => ("raw_json_string", s),
            parse::Value::RawJsonFile(ref path) => ("raw_json_file", path),
            parse::Value::UrlParam(ref s)       => ("url_param", s),
        };
        map.insert("type".to_string(), json::Value::String(ty.to_string()));
        map.insert("value".to_string(), json::Value::String(value.to_string()));
        json::Value::Object(map)
    }
}

pub fn to_json_for_dump(pairs: &[parse::Pair]) -> String {
    let kvs = pairs.iter().map(|pair| {
        let k = json::Value::String(pair.0.clone());
        let v = pair.1.clone().map(|v| v.to_json_for_dump()).unwrap_or(json::Value::Null);
        json::Value::Array(vec![k, v])
    }).collect();
    let root = json::Value::Array(kvs);
    json::ser::to_string(&root).unwrap_or("".to_string())
}
