use std::fmt;
use std::marker::PhantomData;

extern crate serde;
use self::serde::de::{Deserialize, DeserializeSeed, Deserializer, Visitor,
                      MapAccess, IgnoredAny, Error};

/// Extract a single value from a nested path within a struct.
pub struct Extract<'a, T> {
    path: &'a [&'a str],
    ty: PhantomData<T>,
}

impl<'a, T> Extract<'a, T> {
    pub fn new(path: &'a [&'a str]) -> Self {
        assert!(!path.is_empty());
        Extract { path, ty: PhantomData }
    }
}

impl<'de, 'a, T> DeserializeSeed<'de> for Extract<'a, T>
    where T: Deserialize<'de>
{
    type Value = Option<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_map(self)
    }
}

#[derive(Copy, Clone)]
struct ExpectedKey<'a>(&'a str);

impl<'de, 'a> DeserializeSeed<'de> for ExpectedKey<'a> {
    type Value = bool;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de, 'a> Visitor<'de> for ExpectedKey<'a> {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "struct field `{}`", self.0)
    }

    fn visit_str<E>(self, s: &str) -> Result<bool, E>
        where E: Error
    {
        Ok(s == self.0)
    }
}

impl<'de, 'a, T> Visitor<'de> for Extract<'a, T>
    where T: Deserialize<'de>
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "struct with field `{}`", self.path[0])
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where M: MapAccess<'de>
    {
        let mut value = None;
        let first = ExpectedKey(self.path[0]);
        while let Some(expected) = map.next_key_seed(first)? {
            if expected {
                if self.path.len() == 1 {
                    value = Some(map.next_value()?);
                } else {
                    value = map.next_value_seed(Self::new(&self.path[1..]))?;
                }
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Extract;
    use serde::de::DeserializeSeed;
    use serde_json;

    #[test]
    fn test_extract() {
        let j = r#" {"_id": {"$oid": "xx"}, "total_money_raised": "$39.8M"} "#;
        {
            let path = &["_id", "$oid"];
            let mut de = serde_json::Deserializer::from_str(j);
            let v = Extract::<&str>::new(path).deserialize(&mut de).unwrap();
            assert_eq!(v, Some("xx"));
        }
        {
            let path = &["total_money_raised"];
            let mut de = serde_json::Deserializer::from_str(j);
            let v = Extract::<&str>::new(path).deserialize(&mut de).unwrap();
            assert_eq!(v, Some("$39.8M"));
        }
        {
            let path = &["missing"];
            let mut de = serde_json::Deserializer::from_str(j);
            let v = Extract::<&str>::new(path).deserialize(&mut de).unwrap();
            assert_eq!(v, None);
        }
    }
}
