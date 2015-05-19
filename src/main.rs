extern crate clap;
extern crate serde;

mod parse;
mod serialize;

use clap::{App, Arg};

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
        println!("{}", serialize::to_json_for_dump(&pairs));
    } else {
        println!("{}", serialize::to_json(&pairs));
    }
}
