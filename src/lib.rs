#![feature(fn_must_use)]
extern crate json;
extern crate pikkr;
extern crate serde_json;
extern crate fnv;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::{AddAssign, Div};
use std::time::{Duration, Instant};
use fnv::FnvHashMap;

trait Parser: Sized {
    fn parse(&mut self, rec: &[u8], queries: &Vec<&[u8]>, print: bool) -> (usize, Duration);
}

struct JsonParser {}

impl Parser for JsonParser {
    fn parse(&mut self, rec: &[u8], queries: &Vec<&[u8]>, print: bool) -> (usize, Duration) {
        let s = &String::from_utf8(rec.to_vec()).unwrap();
        let mut qs = Vec::new();
        for q in queries {
            let mut b = false;
            for i in 2..q.len() {
                if q[i] == 0x2e {
                    qs.push(vec![
                        String::from_utf8(q.get(2..i).unwrap().to_vec()).unwrap(),
                        String::from_utf8(q.get(i+1..q.len()).unwrap().to_vec()).unwrap(),
                    ]);
                    b = true;
                    break;
                }
            }
            if b {
                continue;
            }
            qs.push(vec![String::from_utf8(q.get(2..q.len()).unwrap().to_vec()).unwrap()]);
        }
        let mut r = 0;
        let now = Instant::now();
        let v = json::parse(s).unwrap();
        for q in qs {
            let res = if q.len() == 1 {
                &v[&q[0]]
            } else {
                &v[&q[0]][&q[1]]
            }.to_string();
            r += res.len();
            if print {
                println!("{}", res);
            }
        }
        let elapsed = now.elapsed();
        (r, elapsed)
    }
}

struct PikkrParser<'a> {
    pikkr: pikkr::pikkr::Pikkr<'a>,
}

impl<'a> Parser for PikkrParser<'a> {
    fn parse(&mut self, rec: &[u8], _: &Vec<&[u8]>, print: bool) -> (usize, Duration) {
        let mut r = 0;
        let now = Instant::now();
        let v = self.pikkr.parse(rec);
        for x in v {
            let x = x.unwrap();
            r += x.len();
            if print {
                println!("{}", x);
            }
        }
        let elapsed = now.elapsed();
        (r, elapsed)
    }
}

struct SerdeJsonParser {}

impl Parser for SerdeJsonParser {
    fn parse(&mut self, rec: &[u8], queries: &Vec<&[u8]>, print: bool) -> (usize, Duration) {
        let mut qs = Vec::new();
        for q in queries {
            let mut b = false;
            for i in 2..q.len() {
                if q[i] == 0x2e {
                    qs.push(vec![
                        String::from_utf8(q.get(2..i).unwrap().to_vec()).unwrap(),
                        String::from_utf8(q.get(i+1..q.len()).unwrap().to_vec()).unwrap(),
                    ]);
                    b = true;
                    break;
                }
            }
            if b {
                continue;
            }
            qs.push(vec![String::from_utf8(q.get(2..q.len()).unwrap().to_vec()).unwrap()]);
        }
        let mut r = 0;
        let now = Instant::now();
        let v: serde_json::Value = serde_json::from_slice(rec).unwrap();
        for q in qs {
            let res = if q.len() == 1 {
                &v[&q[0]]
            } else {
                &v[&q[0]][&q[1]]
            }.to_string();
            r += res.len();
            if print {
                println!("{}", res);
            }
        }
        let elapsed = now.elapsed();
        (r, elapsed)
    }
}

pub struct Executor {
    file_path: String,
    parser_name: String,
    queries: String,
    print: bool,
    train_num: usize
}

impl Executor {
    pub fn new(args: &[String]) -> Executor {
        Executor {
            file_path: args[1].clone(),
            parser_name: args[2].clone(),
            queries: args[3].clone(),
            print: args.len() > 4 && args[4] == "true",
            train_num: if args.len() > 5 { args[5].parse().unwrap() } else { 1000000000000000000 }
        }
    }

    pub fn run(&self) {
        println!("file_path: {}, parser_name: {}, queries: {} print: {} train_num: {}", self.file_path, self.parser_name, self.queries, self.print, self.train_num);
        match self.parser_name.as_ref() {
           "json" => self.parse(JsonParser {}),
           "serde_json" => self.parse( SerdeJsonParser {}),
           "pikkr" => {
               let mut query_strs = vec![];
               for s in self.queries.split(",") {
                   query_strs.push(s.as_bytes());
               }
               let queries = FnvHashMap::default();
               let stats = FnvHashMap::default();
               let p = pikkr::pikkr::Pikkr::new(self.train_num, &query_strs, queries, stats);
               let parser = PikkrParser{pikkr: p};
               self.parse(parser)
           },
            _ => (),
        };
    }

    fn parse<T: Parser>(&self, mut parser: T) {
        let f = File::open(&self.file_path).expect("");
        let f = BufReader::new(&f);
        let mut queries = vec![];
        for s in self.queries.split(",") {
            queries.push(s.as_bytes());
        }
        let mut num = 0;
        let mut size = 0;
        let mut r = 0;
        let mut elapsed = Duration::new(0, 0);
        for (_, l) in f.lines().enumerate() {
            let l = l.unwrap();
            let b = l.as_bytes();
            let res = parser.parse(b, &queries, self.print);
            num += 1;
            size += b.len();
            r += res.0;
            elapsed.add_assign(res.1);
        }
        println!("num: {}, size: {}, r: {}, elapsed: {:?}, average size: {}, average elapsed: {:?}", num, size, r, elapsed, size / num, elapsed.div(num as u32));
    }
}
