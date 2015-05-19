extern crate clap;
extern crate serde;

use clap::{App, Arg};
use std::collections::btree_map::BTreeMap;
use std::fs::File;
use std::io::Read;
use serde::json;

mod parse;

fn slurp(path: &str) -> Option<String> {
    File::open(path).ok().as_mut().and_then(|f| {
        let mut s = String::new();
        f.read_to_string(&mut s).ok().and_then(|_| Some(s))
    })
}

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

fn to_json(pairs: &[parse::Pair]) -> String {
    let mut map = BTreeMap::new();
    for pair in pairs.iter() {
        let k = pair.0.clone();
        let v = pair.1.clone().map(|v| v.to_json()).unwrap_or(json::Value::Null);
        map.insert(k, v);
    }
    let root = json::Value::Object(map);
    json::ser::to_string(&root).unwrap_or("".to_string())
}

fn dump(pairs: &[parse::Pair]) -> String {
    let kvs = pairs.iter().map(|pair| {
        let k = json::Value::String(pair.0.clone());
        let v = pair.1.clone().map(|v| v.to_json_for_dump()).unwrap_or(json::Value::Null);
        json::Value::Array(vec![k, v])
    }).collect();
    let root = json::Value::Array(kvs);
    json::ser::to_string(&root).unwrap_or("".to_string())
}

fn main() {
    let matches = App::new("skv")
        .version("0.1.0")
        .arg(Arg::with_name("dump")
            .long("dump")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("key-value pairs")
            .index(1)
            .multiple(true))
        .get_matches();
    let items = matches.values_of("key-value pairs").unwrap_or(vec![]);
    let pairs = parse::parse(&items);
    if matches.is_present("dump") {
        println!("{}", dump(&pairs));
    } else {
        println!("{}", to_json(&pairs));
    }
}
