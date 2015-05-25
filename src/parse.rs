extern crate regex;

use self::regex::Regex;

const SEP_DATA_FILE       : &'static str = "=@";
const SEP_DATA_STRING     : &'static str = "=";
const SEP_FORM_FILE       : &'static str = "@";
const SEP_HTTP_HEADER     : &'static str = ":";
const SEP_RAW_JSON_FILE   : &'static str = ":=@";
const SEP_RAW_JSON_STRING : &'static str = ":=";
const SEP_URL_PARAM       : &'static str = "==";

pub type Pair = (String, Option<Value>);

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    DataFile(String),
    DataString(String),
    FormFile(String),
    HttpHeader(String),
    RawJsonString(String),
    RawJsonFile(String),
    UrlParam(String),
}

impl Value {
    fn from_str_and_sep(s: &str, sep: &str) -> Option<Value> {
        let s = s.to_string();
        match sep {
            SEP_DATA_FILE       => Some(Value::DataFile(s)),
            SEP_DATA_STRING     => Some(Value::DataString(s)),
            SEP_FORM_FILE       => Some(Value::FormFile(s)),
            SEP_HTTP_HEADER     => Some(Value::HttpHeader(s)),
            SEP_RAW_JSON_FILE   => Some(Value::RawJsonFile(s)),
            SEP_RAW_JSON_STRING => Some(Value::RawJsonString(s)),
            SEP_URL_PARAM       => Some(Value::UrlParam(s)),
            _                   => None
        }
    }
}

pub fn parse<T>(items: &[T]) -> Vec<Pair>
where T: AsRef<str> {
    items.iter().map(|item|
        pair_from_tokens(tokenize(item.as_ref()))
    ).collect()
}

#[derive(Debug)]
enum Token {
    Escaped(String),
    String(String),
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let re = Regex::new(r"(?s)([^\\]+)|\\(.|$)").unwrap();
    for cap in re.captures_iter(s) {
        if let Some(c1) = cap.at(1) {
            tokens.push(Token::String(c1.to_string()));
        }
        if let Some(c2) = cap.at(2) {
            tokens.push(Token::Escaped(c2.to_string()));
        }
    }
    tokens
}

fn pair_from_tokens(tokens: Vec<Token>) -> Pair {
    let mut key = String::new();
    let mut value = String::new();
    let mut sep = String::new();
    let sep_re = Regex::new(r"(?s)(.*?)(:=@|:=|==|=@|:|=|@)(.*)").unwrap();
    let mut tokens_iter = tokens.into_iter();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Escaped(ref s) => {
                key.push_str(s);
            }
            Token::String(ref s) => {
                if let Some(cap) = sep_re.captures(s) {
                    key.push_str(&cap.at(1).unwrap());
                    sep = cap.at(2).unwrap().to_string();
                    value.push_str(&cap.at(3).unwrap());
                    break;
                } else {
                    key.push_str(s);
                }
            }
        }
    }
    for token in tokens_iter {
        match token {
            Token::Escaped(ref s) => value.push_str(s),
            Token::String(ref s)  => value.push_str(s),
        }
    }
    (key, Value::from_str_and_sep(&value, &sep))
}
