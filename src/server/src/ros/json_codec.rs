//! Conversion between the bridge's JSON wire format and `rclrs::DynamicMessage`.
//!
//! JSON is deliberately kept at this boundary: ROS entities only ever receive or
//! produce `DynamicMessage`s, while gRPC only ever receives or produces UTF-8 JSON.

use rclrs::{
    ArrayValue, DynamicMessage, DynamicMessageView, DynamicMessageViewMut, MessageTypeName,
    SequenceValue, SequenceValueMut, SimpleValue, SimpleValueMut, Value, ValueMut,
};
use serde_json::{Map, Value as JsonValue};

pub fn from_json(message_type: MessageTypeName, json: &[u8]) -> Result<DynamicMessage, String> {
    let value: JsonValue =
        serde_json::from_slice(json).map_err(|error| format!("invalid JSON: {error}"))?;
    let object = value
        .as_object()
        .ok_or("a ROS message must be a JSON object")?;
    let mut message = DynamicMessage::new(message_type).map_err(|error| error.to_string())?;
    fill_message(&mut message, object)?;
    Ok(message)
}

pub fn to_json(message: &DynamicMessage) -> Result<Vec<u8>, String> {
    serde_json::to_vec(&message_to_json(message)).map_err(|error| error.to_string())
}

fn message_to_json(message: &DynamicMessage) -> JsonValue {
    JsonValue::Object(
        message
            .iter()
            .map(|(name, value)| (name.to_owned(), value_to_json(value)))
            .collect(),
    )
}

fn view_to_json(view: &DynamicMessageView<'_>) -> JsonValue {
    JsonValue::Object(
        view.iter()
            .map(|(name, value)| (name.to_owned(), value_to_json(value)))
            .collect(),
    )
}

fn json_number_or_string<T: serde::Serialize>(value: T) -> JsonValue {
    serde_json::to_value(value).unwrap_or(JsonValue::Null)
}

fn simple_to_json(value: SimpleValue<'_>) -> JsonValue {
    match value {
        SimpleValue::Boolean(v) => JsonValue::Bool(*v),
        SimpleValue::String(v) => JsonValue::String(v.to_string()),
        SimpleValue::BoundedString(v) => JsonValue::String(v.to_string()),
        SimpleValue::WString(v) => JsonValue::String(v.to_string()),
        SimpleValue::BoundedWString(v) => JsonValue::String(v.to_string()),
        SimpleValue::Message(v) => view_to_json(&v),
        SimpleValue::LongDouble(_) => JsonValue::Null,
        SimpleValue::Float(v) => json_number_or_string(*v),
        SimpleValue::Double(v) => json_number_or_string(*v),
        SimpleValue::Char(v) | SimpleValue::Octet(v) | SimpleValue::Uint8(v) => {
            json_number_or_string(*v)
        }
        SimpleValue::WChar(v) | SimpleValue::Uint16(v) => json_number_or_string(*v),
        SimpleValue::Int8(v) => json_number_or_string(*v),
        SimpleValue::Int16(v) => json_number_or_string(*v),
        SimpleValue::Uint32(v) => json_number_or_string(*v),
        SimpleValue::Int32(v) => json_number_or_string(*v),
        // JSON clients must not lose 64-bit precision: represent them as decimal strings.
        SimpleValue::Uint64(v) => JsonValue::String(v.to_string()),
        SimpleValue::Int64(v) => JsonValue::String(v.to_string()),
    }
}

fn array_to_json(value: ArrayValue<'_>) -> JsonValue {
    match value {
        ArrayValue::BooleanArray(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::Bool(*x)).collect())
        }
        ArrayValue::StringArray(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        ArrayValue::BoundedStringArray(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        ArrayValue::WStringArray(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        ArrayValue::BoundedWStringArray(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        ArrayValue::MessageArray(v) => JsonValue::Array(v.iter().map(view_to_json).collect()),
        ArrayValue::LongDoubleArray(_, _) => JsonValue::Null,
        ArrayValue::Uint64Array(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        ArrayValue::Int64Array(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        ArrayValue::FloatArray(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::DoubleArray(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::CharArray(v) | ArrayValue::OctetArray(v) | ArrayValue::Uint8Array(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::WCharArray(v) | ArrayValue::Uint16Array(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::Int8Array(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::Int16Array(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::Uint32Array(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
        ArrayValue::Int32Array(v) => {
            JsonValue::Array(v.iter().map(|x| json_number_or_string(*x)).collect())
        }
    }
}

fn sequence_to_json(value: SequenceValue<'_>) -> JsonValue {
    match value {
        SequenceValue::BooleanSequence(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::Bool(*x)).collect())
        }
        SequenceValue::StringSequence(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        SequenceValue::BoundedStringSequence(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        SequenceValue::WStringSequence(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        SequenceValue::BoundedWStringSequence(v) => {
            JsonValue::Array(v.iter().map(|x| JsonValue::String(x.to_string())).collect())
        }
        SequenceValue::MessageSequence(v) => JsonValue::Array(v.iter().map(view_to_json).collect()),
        SequenceValue::LongDoubleSequence(_) => JsonValue::Null,
        SequenceValue::Uint64Sequence(v) => json_strings(v),
        SequenceValue::Int64Sequence(v) => json_strings(v),
        SequenceValue::FloatSequence(v) => json_values(v),
        SequenceValue::DoubleSequence(v) => json_values(v),
        SequenceValue::CharSequence(v)
        | SequenceValue::OctetSequence(v)
        | SequenceValue::Uint8Sequence(v) => json_values(v),
        SequenceValue::WCharSequence(v) | SequenceValue::Uint16Sequence(v) => json_values(v),
        SequenceValue::Int8Sequence(v) => json_values(v),
        SequenceValue::Int16Sequence(v) => json_values(v),
        SequenceValue::Uint32Sequence(v) => json_values(v),
        SequenceValue::Int32Sequence(v) => json_values(v),
    }
}

fn json_values<T: Copy + serde::Serialize>(values: &[T]) -> JsonValue {
    JsonValue::Array(values.iter().map(|x| json_number_or_string(*x)).collect())
}
fn json_strings<T: ToString>(values: &[T]) -> JsonValue {
    JsonValue::Array(
        values
            .iter()
            .map(|x| JsonValue::String(x.to_string()))
            .collect(),
    )
}

fn value_to_json(value: Value<'_>) -> JsonValue {
    match value {
        Value::Simple(v) => simple_to_json(v),
        Value::Array(v) => array_to_json(v),
        Value::Sequence(v) => sequence_to_json(v),
        Value::BoundedSequence(_) => JsonValue::Null, // handled on input; unsupported output is explicit rather than corrupt.
    }
}

fn fill_message(
    message: &mut DynamicMessage,
    object: &Map<String, JsonValue>,
) -> Result<(), String> {
    for (name, value) in message.iter_mut() {
        if let Some(json) = object.get(name) {
            fill_value(value, json)?;
        }
    }
    Ok(())
}

fn fill_view(
    view: DynamicMessageViewMut<'_>,
    object: &Map<String, JsonValue>,
) -> Result<(), String> {
    for (name, value) in view.iter_mut() {
        if let Some(json) = object.get(name) {
            fill_value(value, json)?;
        }
    }
    Ok(())
}

fn number<T: std::str::FromStr>(value: &JsonValue) -> Result<T, String> {
    let text = value
        .as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| value.to_string());
    text.parse()
        .map_err(|_| format!("expected a number, got {value}"))
}

fn fill_value(value: ValueMut<'_>, json: &JsonValue) -> Result<(), String> {
    match value {
        ValueMut::Simple(v) => fill_simple(v, json),
        ValueMut::Sequence(v) => fill_sequence(v, json),
        ValueMut::Array(_) | ValueMut::BoundedSequence(_) => Err(
            "fixed arrays and bounded sequences are not supported by this JSON codec yet".into(),
        ),
    }
}

fn fill_simple(value: SimpleValueMut<'_>, json: &JsonValue) -> Result<(), String> {
    match value {
        SimpleValueMut::Boolean(v) => *v = json.as_bool().ok_or("expected boolean")?,
        SimpleValueMut::Float(v) => *v = number(json)?,
        SimpleValueMut::Double(v) => *v = number(json)?,
        SimpleValueMut::Char(v) | SimpleValueMut::Octet(v) | SimpleValueMut::Uint8(v) => {
            *v = number(json)?
        }
        SimpleValueMut::WChar(v) | SimpleValueMut::Uint16(v) => *v = number(json)?,
        SimpleValueMut::Int8(v) => *v = number(json)?,
        SimpleValueMut::Int16(v) => *v = number(json)?,
        SimpleValueMut::Uint32(v) => *v = number(json)?,
        SimpleValueMut::Int32(v) => *v = number(json)?,
        SimpleValueMut::Uint64(v) => *v = number(json)?,
        SimpleValueMut::Int64(v) => *v = number(json)?,
        SimpleValueMut::String(v) => *v = json.as_str().ok_or("expected string")?.into(),
        SimpleValueMut::WString(v) => *v = json.as_str().ok_or("expected string")?.into(),
        SimpleValueMut::Message(v) => fill_view(v, json.as_object().ok_or("expected object")?)?,
        SimpleValueMut::BoundedString(_)
        | SimpleValueMut::BoundedWString(_)
        | SimpleValueMut::LongDouble(_) => {
            return Err(
                "bounded strings and long doubles are not supported by this JSON codec yet".into(),
            );
        }
    }
    Ok(())
}

fn fill_sequence(value: SequenceValueMut<'_>, json: &JsonValue) -> Result<(), String> {
    let values = json.as_array().ok_or("expected array")?;
    macro_rules! set_numbers {
        ($sequence:expr) => {{
            *$sequence = rosidl_runtime_rs::Sequence::new(values.len());
            for (slot, json) in $sequence.as_mut_slice().iter_mut().zip(values) {
                *slot = number(json)?;
            }
        }};
    }
    match value {
        SequenceValueMut::FloatSequence(v) => set_numbers!(v), SequenceValueMut::DoubleSequence(v) => set_numbers!(v),
        SequenceValueMut::CharSequence(v) | SequenceValueMut::OctetSequence(v) | SequenceValueMut::Uint8Sequence(v) => set_numbers!(v),
        SequenceValueMut::WCharSequence(v) | SequenceValueMut::Uint16Sequence(v) => set_numbers!(v), SequenceValueMut::Int8Sequence(v) => set_numbers!(v), SequenceValueMut::Int16Sequence(v) => set_numbers!(v),
        SequenceValueMut::Uint32Sequence(v) => set_numbers!(v), SequenceValueMut::Int32Sequence(v) => set_numbers!(v), SequenceValueMut::Uint64Sequence(v) => set_numbers!(v), SequenceValueMut::Int64Sequence(v) => set_numbers!(v),
        SequenceValueMut::BooleanSequence(v) => { *v = rosidl_runtime_rs::Sequence::new(values.len()); for (slot, json) in v.as_mut_slice().iter_mut().zip(values) { *slot = json.as_bool().ok_or("expected boolean")?; } }
        _ => return Err("string, nested-message, bounded, and long-double sequences are not supported by this JSON codec yet".into()),
    }
    Ok(())
}
