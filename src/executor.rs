use targets::Parser;
use targets::json::JsonParser;
use targets::pikkr::PikkrParser;
use targets::serde_json::SerdeJsonParser;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::time::{Duration, Instant};


pub struct Executor {
    file_path: String,
    parser_name: String,
    queries: String,
    print: bool,
    train_num: usize,
}

impl Executor {
    pub fn new(args: &[String]) -> Executor {
        Executor {
            file_path: args[1].clone(),
            parser_name: args[2].clone(),
            queries: args[3].clone(),
            print: args.len() > 4 && args[4] == "true",
            train_num: if args.len() > 5 {
                args[5].parse().unwrap()
            } else {
                1000000000000000000
            },
        }
    }

    pub fn run(&self) {
        println!(
            "file_path: {}, parser_name: {}, queries: {} print: {} train_num: {}",
            self.file_path,
            self.parser_name,
            self.queries,
            self.print,
            self.train_num
        );
        let queries: Vec<_> = self.queries.split(",").collect();
        match self.parser_name.as_ref() {
            "json" => {
                let parser = JsonParser::new(&queries).unwrap();
                self.parse(parser)
            }
            "serde_json" => {
                let parser = SerdeJsonParser::new(&queries);
                self.parse(parser)
            }
            "pikkr" => {
                let parser = PikkrParser::new(&queries, self.train_num).unwrap();
                self.parse(parser)
            }
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
            elapsed += res.1;
        }
        print!(
            "num: {}, size: {}, r: {}, elapsed: {:?}, ",
            num,
            size,
            r,
            elapsed,
        );
        println!(
            "average size: {}, average elapsed: {:?}, throughput (mb/sec): {:.4}",
            size / num,
            elapsed / (num as u32),
            (size as f64 /
                (elapsed.as_secs() as f64 +
                    (elapsed.subsec_nanos() as u64 as f64) / 1000000000f64) / 1024f64 /
                1024f64)
        );
    }
}

fn stopwatch<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let r = f();
    let elapsed = now.elapsed();

    (r, elapsed)
}
