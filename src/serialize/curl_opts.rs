use parse;
use regex::{Captures, Regex};
use url::form_urlencoded;

pub fn to_curl_opts(pairs: &[parse::Pair]) -> Vec<(String, String)> {
    let mut opts = Vec::new();

    let data_string_pairs = pairs.iter().filter_map(|pair| {
        if let Some(parse::Value::DataString(ref s)) = pair.1 {
            Some((pair.0.clone(), shell_escape(s)))
        } else {
            None
        }
    });
    let urlencoded = form_urlencoded::serialize(data_string_pairs);
    if !urlencoded.is_empty() {
        opts.push(("-d".to_string(), urlencoded));
    }

    for pair in pairs {
        if let Some(parse::Value::FormFile(ref path)) = pair.1 {
            let value = format!(r"{}\=@'{}'", shell_escape(&pair.0), shell_escape_inside_single_quote(path));
            opts.push(("-F".to_string(), value));
        } else if let Some(parse::Value::HttpHeader(ref s)) = pair.1 {
            let value = format!(r"'{}: {}'", shell_escape_inside_single_quote(&pair.0), shell_escape_inside_single_quote(s));
            opts.push(("-H".to_string(), value));
        }
    }

    opts
}

pub fn to_curl_opts_string(pairs: &[parse::Pair]) -> String {
    to_curl_opts(pairs)
        .iter()
        .map(|opt_val| {
            format!("{} {}", opt_val.0, opt_val.1)
        })
        .collect::<Vec<_>>()
        .connect(" ")
}

fn shell_escape(s: &str) -> String {
    // Adapted from Ruby's `Shellwords::shellescape()`.
    if s.is_empty() {
        return "''".to_string();
    }
    let re = Regex::new(r"[^0-9A-Za-z_.,:/@=\n-]").unwrap();
    let s = re.replace(s, |cap: &Captures| {
        let c0 = cap.at(0).unwrap();
        format!(r"\\{}", c0)
    });
    let s = s.replace("\n", "'\n'");
    s
}

fn shell_escape_inside_single_quote(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }
    let s = s.replace(r"\", r"\\");
    let s = s.replace(r"'", r"'\''");
    s
}
