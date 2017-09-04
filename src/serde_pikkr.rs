extern crate linear_map;

use self::linear_map::LinearMap as Map;
use std::{fmt, str};
use serde::de::{DeserializeSeed, Deserializer, Visitor, MapAccess, IgnoredAny, Error};
use serde_json::{self, Value};

pub struct Pikkr<'a> {
    /// Map of top-level keys that we care about.
    queries: Queries<'a>,

    /// Total number of queries. May be more than `self.queries.len()` if
    /// multiple queries share a common prefix.
    len: usize,
}

/// Map of keys that we care about.
type Queries<'a> = Map<&'a str, Query<'a>>;

/// Corresponds to a key we care about, either because we need its value or
/// because we care about some nested keys.
#[derive(Clone)]
struct Query<'a> {
    /// Map of keys nested under this one that we care about.
    subqueries: Queries<'a>,

    /// Index in the output array to write the result of this query. May be
    /// `None` if we only need nested values, not this one.
    output: Option<usize>,
}

impl<'a> Pikkr<'a> {
    /// Paths look like "$.a.b.c". The corresponding values will be written into
    /// the output array in the same order, i.e. the result for `paths[0]` ends
    /// up in `output[0]`.
    pub fn new(paths: &[&'a str]) -> Self {
        let mut q = Query { subqueries: Queries::new(), output: None };
        for (i, path) in paths.into_iter().enumerate() {
            let mut cur = &mut q;
            for elem in path[2..].split('.') {
                let cur1 = cur; // borrow checker workaround #10520
                cur = cur1.subqueries.entry(elem).or_insert_with(|| {
                    Query { subqueries: Queries::new(), output: None }
                });
            }
            cur.output = Some(i);
        }
        Pikkr {
            queries: q.subqueries,
            len: paths.len(),
        }
    }

    pub fn parse(&self, rec: &[u8]) -> Vec<Option<Value>> {
        let mut output = vec![None; self.len];
        Parse { queries: self.queries.clone(), output: &mut output }
            .deserialize(&mut serde_json::Deserializer::from_slice(rec))
            .unwrap();
        output
    }
}

struct Parse<'a> {
    queries: Queries<'a>,
    output: &'a mut [Option<Value>],
}

impl<'de, 'a> DeserializeSeed<'de> for Parse<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_map(self)
    }
}

/// Pick one key out of the map, producing its corresponding Query when
/// deserialized.
struct Key<'r, 'a: 'r>(&'r mut Queries<'a>);

impl<'de, 'r, 'a> DeserializeSeed<'de> for Key<'r, 'a> {
    type Value = Option<Query<'a>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de, 'r, 'a> Visitor<'de> for Key<'r, 'a> {
    type Value = Option<Query<'a>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "one of {} possible struct fields", self.0.len())
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where E: Error
    {
        Ok(self.0.remove(s))
    }
}

impl<'de, 'a> Visitor<'de> for Parse<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "JSON map")
    }

    fn visit_map<M>(mut self, mut map: M) -> Result<Self::Value, M::Error>
        where M: MapAccess<'de>
    {
        // Iterate through keys in the map.
        while let Some(key) = map.next_key_seed(Key(&mut self.queries))? {
            // Decide whether we care about this key.
            if let Some(query) = key {
                // See whether the full content of this entry needs to be saved.
                if let Some(i) = query.output {
                    // Read the full content as a serde_json::Value.
                    let v = map.next_value()?;
                    // Deserialize any nested values that were also requested.
                    if !query.subqueries.is_empty() {
                        let sub = Parse {
                            queries: query.subqueries,
                            output: self.output,
                        };
                        sub.deserialize(&v).map_err(Error::custom)?;
                    }
                    // Save the value into the output.
                    self.output[i] = Some(v);
                } else {
                    // Deserialize nested values that were requested.
                    let sub = Parse {
                        queries: query.subqueries,
                        output: self.output,
                    };
                    map.next_value_seed(sub)?;
                }
            } else {
                // Ignore everything under this key.
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Pikkr;

    const JSON: &[u8] = br#"{
                                "_id": {"$oid": "52cdef7c4bab8bd675297d8a"},
                                "total_money_raised": "$39.8M"
                            }"#;

    #[test]
    fn test_strings() {
        let paths = &["$._id.$oid", "$.total_money_raised"];
        let expected = vec![
            Some(json!("52cdef7c4bab8bd675297d8a")),
            Some(json!("$39.8M")),
        ];
        assert_eq!(Pikkr::new(paths).parse(JSON), expected);
    }

    #[test]
    fn test_object() {
        let paths = &["$._id"];
        let expected = vec![
            Some(json!({ "$oid": "52cdef7c4bab8bd675297d8a" })),
        ];
        assert_eq!(Pikkr::new(paths).parse(JSON), expected);
    }

    #[test]
    fn test_nested() {
        let paths = &["$._id", "$._id.$oid"];
        let expected = vec![
            Some(json!({ "$oid": "52cdef7c4bab8bd675297d8a" })),
            Some(json!("52cdef7c4bab8bd675297d8a")),
        ];
        assert_eq!(Pikkr::new(paths).parse(JSON), expected);
    }

    #[test]
    fn test_missing() {
        let paths = &["$.missing"];
        let expected = vec![None];
        assert_eq!(Pikkr::new(paths).parse(JSON), expected);
    }
}
