use std::{error::Error, os::unix::process::parent_id};

use cl_parser::parser::{
    self,
    parse::{parse, ParseMode},
};

fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let parser = parse(s.clone(), ParseMode::Fuzzy);

        match parser {
            Ok(c) => {
                dbg!(c);
            }
            Err(parser::parse::Error::EmptySource) => {
                println!("Empty");
            }
            _ => unreachable!(),
        };
    }
}
