
use bytes::Bytes;
use serde::{
    de::{self, Error},
    Deserialize,
};

pub mod hexbytes {
    use serde::Serializer;

    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(hex::decode(s.strip_prefix("0x").unwrap_or(&s))
            .map_err(D::Error::custom)?
            .into())
    }

    pub fn serialize<S>(b: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(b)))
    }
}


pub mod duration_as_millis {
    use super::*;
    use serde::Serializer;
    use std::time::Duration;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        Ok(Duration::from_millis(u64::deserialize(deserializer)?))
    }

    pub fn serialize<S>(d: &Duration, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u64(d.as_millis() as u64)
    }
}