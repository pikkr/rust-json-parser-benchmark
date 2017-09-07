use std::str;
use pikkr::{Pikkr, Result};
use super::Parser;


pub struct PikkrParser<'a> {
    pikkr: Pikkr<'a>,
}

impl<'a> PikkrParser<'a> {
    pub fn new(queries: &[&'a str], train_num: usize) -> Result<Self> {
        let pikkr = Pikkr::new(queries, train_num)?;
        Ok(PikkrParser { pikkr })
    }
}

impl<'a> Parser for PikkrParser<'a> {
    fn parse(&mut self, rec: &str, print: bool) -> usize {
        let v = self.pikkr.parse(rec.as_bytes()).unwrap();

        let mut r = 0;
        for x in v {
            let x = unsafe { str::from_utf8_unchecked(x.unwrap()) };
            r += x.len();
            if print {
                println!("{}", x);
            }
        }
        r
    }
}
