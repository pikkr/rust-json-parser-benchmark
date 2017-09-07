use std::str;
use json::{self, JsonValue};
use super::Parser;


#[derive(Debug)]
pub enum JsonParserError {
    InvalidQuery,
}

pub struct JsonParser<'q> {
    queries: Vec<Query<'q>>,
}

impl<'q> JsonParser<'q> {
    pub fn new(queries: &[&'q str]) -> Result<Self, JsonParserError> {
        let mut qs = Vec::new();
        for q in queries {
            qs.push(Query::from_str(q)?);
        }

        Ok(Self { queries: qs })
    }

    fn _parse(&self, rec: &str) -> Vec<Option<JsonValue>> {
        let v = json::parse(rec).unwrap();
        self.queries.iter().map(|q| q.get(&v).cloned()).collect()
    }
}

impl<'q> Parser for JsonParser<'q> {
    fn parse(&mut self, rec: &str, print: bool) -> usize {
        let v = self._parse(rec);

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

struct Query<'a> {
    inner: Vec<&'a str>,
}

impl<'a> Query<'a> {
    fn from_str(q: &'a str) -> Result<Self, JsonParserError> {
        if !q.starts_with("$.") {
            return Err(JsonParserError::InvalidQuery);
        }
        let inner: Vec<_> = q[2..].split(".").filter(|s| s.len() > 0).collect();
        if inner.len() == 0 {
            return Err(JsonParserError::InvalidQuery);
        }
        Ok(Query { inner })
    }

    fn get<'v>(&self, mut v: &'v JsonValue) -> Option<&'v JsonValue> {
        for i in 0..(self.inner.len()) {
            if !v.is_object() || !v.has_key(self.inner[i]) {
                return None;
            }
            v = &v[self.inner[i]];
        }
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::Query;
    use json::JsonValue;

    #[test]
    fn test_query_case1() {
        let q = "$._id.$oid";
        let Query { inner } = Query::from_str(q).unwrap();
        assert_eq!(inner, vec!["_id", "$oid"]);
    }

    #[test]
    fn test_query_case2() {
        assert!(Query::from_str("$.").is_err());
    }

    #[test]
    fn test_query_get() {
        let v = object!{
            "_id" => object!{
                "$oid" => "foo"
            }
        };
        let q = Query::from_str("$._id.$oid").unwrap();
        assert_eq!(q.get(&v), Some(&JsonValue::from("foo")));
    }

    #[test]
    fn test_query_get2() {
        let v = object!{
            "_id" => object!{
                "$oid" => "foo"
            }
        };
        let q = Query::from_str("$._id.$oid.missing").unwrap();
        assert_eq!(q.get(&v), None);
    }

    #[test]
    fn test_query_get3() {
        let v = object!{
            "_id" => object!{
                "$oid" => "foo"
            }
        };
        let q = Query::from_str("$._id").unwrap();
        assert_eq!(q.get(&v), Some(&object!{ "$oid" => "foo" }));
    }

    #[test]
    fn test_query_get_missing() {
        let v = object!{
            "_id" => object!{
                "$oid" => "foo"
            }
        };
        let q = Query::from_str("$.missing").unwrap();
        assert_eq!(q.get(&v), None);
    }
}
