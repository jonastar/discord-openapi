use serde::de::{IgnoredAny, Visitor};

mod generated;

pub struct IntTagVisitor {
    field_name: String,
}

impl<'de> Visitor<'de> for IntTagVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("object with integer tag ")?;
        formatter.write_str(&self.field_name)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        loop {
            let key: &str = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(err) => return Err(err),
            };

            if key == self.field_name {
                let next_val: i32 = map.next_value()?;
                tag = Some(next_val)
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        tag.ok_or_else(|| serde::de::Error::custom(format!("missing field '{}'", self.field_name)))
    }
}

pub struct StringTagVisitor {
    field_name: String,
}

impl<'de> Visitor<'de> for StringTagVisitor {
    type Value = &'de str;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("object with integer tag ")?;
        formatter.write_str(&self.field_name)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        loop {
            let key: &str = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(err) => return Err(err),
            };

            if key == self.field_name {
                let next_val: &'de str = map.next_value()?;
                tag = Some(next_val)
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        tag.ok_or_else(|| serde::de::Error::custom(format!("missing field '{}'", self.field_name)))
    }
}
