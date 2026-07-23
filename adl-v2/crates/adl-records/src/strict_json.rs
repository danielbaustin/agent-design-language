use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use serde_json::{Number, Value};

use crate::{ErrorCode, Limits, RecordError, Result};

pub(crate) fn decode(bytes: &[u8], limits: &Limits) -> Result<Value> {
    if bytes.len() > limits.max_envelope_bytes {
        return Err(RecordError::new(
            ErrorCode::Bounds,
            "incoming envelope bound exceeded",
        ));
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let mut members = 0usize;
    let value = StrictSeed {
        depth: 0,
        limits,
        members: &mut members,
    }
    .deserialize(&mut deserializer)
    .map_err(|error| {
        if error.to_string().contains("duplicate object key") {
            RecordError::new(ErrorCode::DuplicateField, "duplicate object key")
        } else {
            RecordError::new(ErrorCode::InvalidEnvelope, "strict JSON decoding failed")
        }
    })?;
    deserializer
        .end()
        .map_err(|_| RecordError::new(ErrorCode::InvalidEnvelope, "trailing channel bytes"))?;
    Ok(value)
}

struct StrictSeed<'a> {
    depth: usize,
    limits: &'a Limits,
    members: &'a mut usize,
}

impl<'de> DeserializeSeed<'de> for StrictSeed<'_> {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if self.depth > self.limits.max_json_depth {
            return Err(D::Error::custom("JSON depth exceeded"));
        }
        deserializer.deserialize_any(StrictVisitor {
            depth: self.depth,
            limits: self.limits,
            members: self.members,
        })
    }
}

struct StrictVisitor<'a> {
    depth: usize,
    limits: &'a Limits,
    members: &'a mut usize,
}

impl<'de> Visitor<'de> for StrictVisitor<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded JSON without duplicate object keys or floats")
    }

    fn visit_unit<E>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_none<E>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_bool<E>(self, value: bool) -> std::result::Result<Value, E> {
        Ok(Value::Bool(value))
    }
    fn visit_u64<E>(self, value: u64) -> std::result::Result<Value, E> {
        Ok(Value::Number(Number::from(value)))
    }
    fn visit_i64<E>(self, value: i64) -> std::result::Result<Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_f64<E>(self, _value: f64) -> std::result::Result<Value, E>
    where
        E: serde::de::Error,
    {
        Err(E::custom("floating point is forbidden"))
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Value, E>
    where
        E: serde::de::Error,
    {
        if value.len() > self.limits.max_string_bytes {
            return Err(E::custom("string bound exceeded"));
        }
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&value)
    }

    fn visit_seq<A>(self, mut sequence: A) -> std::result::Result<Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(StrictSeed {
            depth: self.depth + 1,
            limits: self.limits,
            members: self.members,
        })? {
            charge(self.members, self.limits)?;
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = BTreeMap::new();
        let mut keys = BTreeSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if key.len() > self.limits.max_string_bytes {
                return Err(A::Error::custom("object key bound exceeded"));
            }
            if !keys.insert(key.clone()) {
                return Err(A::Error::custom("duplicate object key"));
            }
            charge(self.members, self.limits)?;
            let value = map.next_value_seed(StrictSeed {
                depth: self.depth + 1,
                limits: self.limits,
                members: self.members,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values.into_iter().collect()))
    }
}

fn charge<E: serde::de::Error>(members: &mut usize, limits: &Limits) -> std::result::Result<(), E> {
    *members = members
        .checked_add(1)
        .ok_or_else(|| E::custom("member count overflow"))?;
    if *members > limits.max_json_members {
        return Err(E::custom("member bound exceeded"));
    }
    Ok(())
}
