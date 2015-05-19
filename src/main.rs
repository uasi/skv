extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("skv")
        .version("0.1.0")
        .arg(Arg::with_name("key-value pairs")
            .index(1)
            .multiple(true))
        .get_matches();
    let pairs = matches.values_of("key-value pairs").unwrap_or(vec![]);
    println!("pairs: {:?}", pairs);
}
