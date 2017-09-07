use std::str;
use json::{self, JsonValue};
use super::Parser;


pub struct JsonParser<'q> {
    queries: Vec<Query<'q>>,
}

impl<'q> JsonParser<'q> {
    pub fn new(queries: &[&'q str]) -> Result<Self, str::Utf8Error> {
        let mut qs = Vec::new();
        for q in queries {
            qs.push(Query::from_slice(q.as_bytes())?);
        }

        Ok(Self { queries: qs })
    }
}

impl<'q> Parser for JsonParser<'q> {
    fn parse(&mut self, rec: &str, print: bool) -> usize {
        let v = json::parse(rec).unwrap();

        let mut r = 0;
        for q in &self.queries {
            let res = q.get(&v).unwrap().to_string();
            r += res.len();
            if print {
                println!("{}", res);
            }
        }
        r
    }
}

struct Query<'a> {
    inner: Vec<&'a str>,
}

impl<'a> Query<'a> {
    fn from_slice(q: &'a [u8]) -> Result<Self, str::Utf8Error> {
        for i in 2..q.len() {
            if q[i] == 0x2e {
                return Ok(Query {
                    inner: vec![
                        str::from_utf8(&q[2..i])?,
                        str::from_utf8(&q[i + 1..q.len()])?,
                    ],
                });
            }
        }
        Ok(Query {
            inner: vec![str::from_utf8(&q[2..q.len()])?],
        })
    }

    fn get<'v>(&self, v: &'v JsonValue) -> Option<&'v JsonValue> {
        if self.inner.len() == 1 {
            Some(&v[self.inner[0]])
        } else {
            Some(&v[self.inner[0]][self.inner[1]])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[test]
    fn test_query_case1() {
        let q = "$._id.$oid";
        let Query { inner } = Query::from_slice(q.as_bytes()).unwrap();
        assert_eq!(inner, vec!["_id", "$oid"]);
    }
}
