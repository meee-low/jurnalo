use json::JsonValue;

use crate::errors::JSONParsingError;

fn get_key_from_object<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Option<&'a JsonValue> {
    match possible_object {
        JsonValue::Object(keys) => keys.get(key),
        _ => None,
    }
}

pub fn get_key_from_object_as_str<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<Option<&'a str>, JSONParsingError> {
    match get_key_from_object(possible_object, key) {
        Some(v) => match v {
            JsonValue::Short(s) => Ok(Some(s.as_str())),
            JsonValue::String(s) => Ok(Some(s)),
            _ => Err(JSONParsingError::UnexpectedTypeForKey),
        },
        None => Ok(None),
    }
}

pub fn get_key_from_object_as_vec<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<Option<&'a Vec<JsonValue>>, JSONParsingError> {
    match get_key_from_object(possible_object, key) {
        Some(v) => match v {
            JsonValue::Array(s) => Ok(Some(s)),
            _ => Err(JSONParsingError::UnexpectedTypeForKey),
        },
        None => Ok(None),
    }
}
