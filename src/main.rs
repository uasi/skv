extern crate clap;
extern crate serde;

use clap::{App, Arg};
use std::collections::btree_map::BTreeMap;
use serde::json;

mod parse;

impl parse::Value {
    fn to_json(&self) -> json::Value {
        let mut map = BTreeMap::new();
        let (ty, value) = match *self {
            parse::Value::DataFile(ref s)      => ("data_file", s),
            parse::Value::DataString(ref s)    => ("data_string", s),
            parse::Value::FormFile(ref s)      => ("form_file", s),
            parse::Value::HttpHeader(ref s)    => ("http_header", s),
            parse::Value::RawJsonString(ref s) => ("raw_json_string", s),
            parse::Value::RawJsonFile(ref s)   => ("raw_json_file", s),
            parse::Value::UrlParam(ref s)      => ("url_param", s),
        };
        map.insert("type".to_string(), json::Value::String(ty.to_string()));
        map.insert("value".to_string(), json::Value::String(value.to_string()));
        json::Value::Object(map)
    }
}

fn dump(kvs: &[(String, Option<parse::Value>)]) -> String {
    let kv_arrays = kvs.iter().map(|kv| {
        let k = json::Value::String(kv.0.clone());
        let v = kv.1.clone().map(|v| v.to_json()).unwrap_or(json::Value::Null);
        json::Value::Array(vec![k, v])
    }).collect();
    let root = json::Value::Array(kv_arrays);
    json::ser::to_string(&root).unwrap_or("".to_string())
}

fn main() {
    let matches = App::new("skv")
        .version("0.1.0")
        .arg(Arg::with_name("key-value pairs")
            .index(1)
            .multiple(true))
        .get_matches();
    let pairs = matches.values_of("key-value pairs").unwrap_or(vec![]);
    let parsed = parse::parse(&pairs);
    println!("dump => {}", dump(&parsed));
}
