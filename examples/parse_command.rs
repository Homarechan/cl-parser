use std::os::unix::process::parent_id;

use cl_parser::parser::parse::{ParseMode, parse};

fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let parser = parse(s.clone(), ParseMode::Fuzzy);
        dbg!(parser.unwrap());
    }

}