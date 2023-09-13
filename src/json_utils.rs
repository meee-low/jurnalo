use json::JsonValue;

use crate::errors::JSONParsingError;

fn get_key_from_object<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<&'a JsonValue, JSONParsingError> {
    match possible_object {
        JsonValue::Object(keys) => match keys.get(key) {
            Some(v) => Ok(v),
            None => Err(JSONParsingError::KeyNotFound(key.to_owned())),
        },
        _ => Err(JSONParsingError::UnexpectedTypeForKey(key.to_owned())),
    }
}

pub fn get_key_from_object_as_str<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<&'a str, JSONParsingError> {
    match get_key_from_object(possible_object, key) {
        Ok(v) => match v {
            JsonValue::Short(s) => Ok(s.as_str()),
            JsonValue::String(s) => Ok(s),
            _ => Err(JSONParsingError::UnexpectedTypeForKey(key.to_owned())),
        },
        Err(e) => Err(e),
    }
}

pub fn get_key_from_object_as_vec<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<&'a Vec<JsonValue>, JSONParsingError> {
    match get_key_from_object(possible_object, key) {
        Ok(v) => match v {
            JsonValue::Array(a) => Ok(a),
            _ => Err(JSONParsingError::UnexpectedTypeForKey(key.to_owned())),
        },
        Err(e) => Err(e),
    }
}
