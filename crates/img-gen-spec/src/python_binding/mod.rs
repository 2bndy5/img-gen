mod validators;

use serde::de::DeserializeOwned;

/// Parse YAML with duplicate-key policy `LastWins` into type `T` by first
/// parsing into `serde_json::Value` (honors LastWins) then converting to `T`.
/// Returns a `String` error on failure for easy mapping to Python exceptions.
pub fn parse_yaml_last_wins<T>(s: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let opts = serde_saphyr::options! {
        duplicate_keys: serde_saphyr::options::DuplicateKeyPolicy::LastWins,
    };
    let value: serde_json::Value =
        serde_saphyr::from_str_with_options(s, opts).map_err(|e| e.to_string())?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}
