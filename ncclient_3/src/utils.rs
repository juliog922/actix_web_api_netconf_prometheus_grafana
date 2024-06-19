use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};

/// A custom error type for handling parsing errors.
#[derive(Debug)]
pub struct Error {}

/// Recursively reads an XML reader and converts the content into a JSON `Value`.
///
/// # Arguments
///
/// * `reader` - A mutable reference to a `Reader` instance.
/// * `depth` - The current depth in the XML tree.
///
/// # Returns
///
/// A `Value` representing the JSON conversion of the XML content.
fn read(reader: &mut Reader<&[u8]>, depth: u64) -> Value {
    let mut buf = Vec::new();
    let mut values = Vec::new();
    let mut node = Map::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if let Ok(name) = String::from_utf8(e.name().into_inner().to_vec()) {
                    let mut child = read(reader, depth + 1);
                    let mut attrs = Map::new();

                    let _ = e
                        .attributes()
                        .map(|a| {
                            if let Ok(attr) = a {
                                let key = String::from_utf8(attr.key.into_inner().to_vec());
                                let value = String::from_utf8(attr.value.to_vec());

                                // Only add the attribute if both key and value are valid utf8
                                if let (Ok(key), Ok(value)) = (key, value) {
                                    let key = format!("@{}", key);
                                    let value = Value::String(value);

                                    // If the child is already an object, insert the attribute there
                                    if child.is_object() {
                                        child.as_object_mut().unwrap().insert(key, value);
                                    } else {
                                        attrs.insert(key, value);
                                    }
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    // Nodes with attributes need special handling
                    if !attrs.is_empty() {
                        if child.is_string() {
                            attrs.insert("#text".to_string(), child);
                        }

                        if let Ok(attrs) = serde_json::to_value(attrs) {
                            node.insert(name, attrs);
                        }
                    } else if node.contains_key(&name) {
                        let (_, mut existing) = node.remove_entry(&name).unwrap();
                        let mut entries: Vec<Value> = vec![];

                        if existing.is_array() {
                            let existing = existing.as_array_mut().unwrap();
                            while !existing.is_empty() {
                                entries.push(existing.remove(0));
                            }
                        } else {
                            entries.push(existing);
                        }
                        entries.push(child);

                        node.insert(name, Value::Array(entries));
                    } else {
                        node.insert(name, child);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Ok(decoded) = e.unescape() {
                    values.push(Value::String(decoded.to_string()));
                }
            }
            Ok(Event::CData(ref e)) => {
                if let Ok(decoded) = e.clone().escape() {
                    if let Ok(decoded_bt) = decoded.unescape() {
                        node.insert("#cdata".to_string(), Value::String(decoded_bt.to_string()));
                    }
                }
            }
            Ok(Event::End(ref _e)) => break,
            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    if !node.is_empty() {
        // Insert collected text if present
        let mut index = 0;
        let mut has_text = false;
        for value in values.iter() {
            if value.is_string() {
                has_text = true;
                break;
            }
            index += 1;
        }

        if has_text {
            node.insert("#text".to_string(), values.remove(index));
        }
        return serde_json::to_value(&node).expect("Failed to convert node to JSON!");
    }

    match values.len() {
        0 => Value::Null,
        1 => values.pop().unwrap(),
        _ => Value::Array(values),
    }
}

/**
 * Converts an XML string into a JSON `Value`.
 *
 * # Arguments
 *
 * * `xml` - A string slice containing the XML data.
 *
 * # Returns
 *
 * A `Result` containing the JSON `Value` or an `Error` if parsing fails.
 */
pub fn to_json(xml: &str) -> Result<Value, Error> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    Ok(read(&mut reader, 0))
}
