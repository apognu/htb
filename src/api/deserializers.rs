use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(super) fn int_or_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    struct StateSuccessVisitor;

    impl<'de> Visitor<'de> for StateSuccessVisitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("unsigned integer or string")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value as u8)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value.parse::<u8>() {
                Ok(value) => self.visit_u8(value),
                Err(_) => Err(E::custom("failed to parse integer")),
            }
        }
    };

    deserializer.deserialize_any(StateSuccessVisitor)
}

pub(super) fn string_or_struct<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    struct ConnectionStatusVisitor<T>(PhantomData<fn() -> Option<T>>);

    impl<'de, T> Visitor<'de> for ConnectionStatusVisitor<T>
    where
        T: Deserialize<'de> + Default,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or connection struct")
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(T::default())
        }

        fn visit_map<M>(self, value: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(MapAccessDeserializer::new(value))
        }
    }

    deserializer.deserialize_any(ConnectionStatusVisitor::<T>(PhantomData))
}
