#![feature(fn_must_use)]
extern crate json;
extern crate pikkr;
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

mod serde_pikkr;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::{AddAssign, Div};
use std::str;
use std::time::{Duration, Instant};

fn stopwatch<F, T>(f: F) -> (T, Duration)
where F: FnOnce() -> T
{
    let now = Instant::now();
    let r = f();
    let elapsed = now.elapsed();
    
    (r, elapsed)
}

trait Parser: Sized {
    fn parse(&mut self, rec: &str, print: bool) -> usize;
}

struct JsonParser<'q> {
    queries: Vec<Vec<&'q str>>,
}

impl<'q> JsonParser<'q> {
    fn new(queries: &[&'q str]) -> Result<Self, str::Utf8Error> {
        let mut qs = Vec::new();
        for q in queries.into_iter().map(|s| s.as_bytes()) {
            let mut b = false;
            for i in 2..q.len() {
                if q[i] == 0x2e {
                    qs.push(vec![
                        str::from_utf8(&q[2..i])?,
                        str::from_utf8(&q[i+1..q.len()])?,
                    ]);
                    b = true;
                    break;
                }
            }
            if b {
                continue;
            }
            qs.push(vec![str::from_utf8(&q[2..q.len()])?]);
        }

        Ok(Self {
            queries: qs,
        })
    }
}

impl<'q> Parser for JsonParser<'q> {
    fn parse(&mut self, rec: &str, print: bool) -> usize {
        let v = json::parse(rec).unwrap();

        let mut r = 0;
        for q in &self.queries {
            let res = if q.len() == 1 {
                &v[q[0]]
            } else {
                &v[q[0]][q[1]]
            }.to_string();
            r += res.len();
            if print {
                println!("{}", res);
            }
        }
        r
    }
}

struct PikkrParser<'a> {
    pikkr: pikkr::Pikkr<'a>,
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

struct SerdeJsonParser<'a> {
    pikkr: serde_pikkr::Pikkr<'a>,
}

impl<'a> Parser for SerdeJsonParser<'a> {
    fn parse(&mut self, rec: &str, print: bool) -> usize {
        let v = self.pikkr.parse(rec.as_bytes());

        let mut r = 0;
        for x in v {
            let x = x.unwrap();
            r += x.to_string().len();
            if print {
                println!("{}", x);
            }
        }
        r
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
           "json" => {
               let queries: Vec<_> = self.queries.split(",").collect();
               let parser = JsonParser::new(&queries).expect("failed to construct queries");
               self.parse(parser)
           },
           "serde_json" => {
               let queries: Vec<_> = self.queries.split(",").collect();
               let p = serde_pikkr::Pikkr::new(&queries);
               self.parse(SerdeJsonParser { pikkr: p })
           }
           "pikkr" => {
               let queries: Vec<_> = self.queries.split(",").map(str::as_bytes).collect();
               let p = pikkr::Pikkr::new(&queries, self.train_num).unwrap();
               let parser = PikkrParser{pikkr: p};
               self.parse(parser)
           },
            _ => (),
        };
    }

    fn parse<T: Parser>(&self, mut parser: T) {
        let f = File::open(&self.file_path).expect("");
        let f = BufReader::new(&f);
        let mut num = 0;
        let mut size = 0;
        let mut r = 0;
        let mut elapsed = Duration::new(0, 0);
        for (_, l) in f.lines().enumerate() {
            let l = l.unwrap();
            let res = stopwatch(|| parser.parse(&l, self.print));
            num += 1;
            size += l.len();
            r += res.0;
            elapsed.add_assign(res.1);
        }
        println!("num: {}, size: {}, r: {}, elapsed: {:?}, average size: {}, average elapsed: {:?}, throughput (mb/sec): {:.4}", num, size, r, elapsed, size / num, elapsed.div(num as u32), (size as f64 / (elapsed.as_secs() as f64 + (elapsed.subsec_nanos() as u64 as f64) / 1000000000f64) / 1024f64 / 1024f64));
    }
}
