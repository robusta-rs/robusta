use serde::{Deserialize, Deserializer};

/// Preserve scalar types even when the deserializer coerces quoted values.
///
/// # Errors
///
/// Returns an error when the value does not deserialize as `T`.
pub fn deserialize_scalar<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    T::deserialize(serde_json::Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
}
