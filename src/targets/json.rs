use std::str;
use json;
use super::Parser;


pub struct JsonParser<'q> {
    queries: Vec<Vec<&'q str>>,
}

impl<'q> JsonParser<'q> {
    pub fn new(queries: &[&'q str]) -> Result<Self, str::Utf8Error> {
        let mut qs = Vec::new();
        for q in queries.into_iter().map(|s| s.as_bytes()) {
            let mut b = false;
            for i in 2..q.len() {
                if q[i] == 0x2e {
                    qs.push(vec![
                        str::from_utf8(&q[2..i])?,
                        str::from_utf8(&q[i + 1..q.len()])?,
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

        Ok(Self { queries: qs })
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
