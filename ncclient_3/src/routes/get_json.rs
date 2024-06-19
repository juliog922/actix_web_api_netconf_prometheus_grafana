use actix_web::{get, web, Responder};
use std::collections::HashMap;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

use crate::netconf::get;
use crate::utils::to_json;
use crate::{AppState, HostParameters};

/// Converts a JSON `Value` to an `Option<f64>`.
/// 
/// # Arguments
/// 
/// * `value` - A reference to the JSON `Value`.
/// 
/// # Returns
/// 
/// An `Option<f64>` which is `Some` if the value can be parsed as a `f64`, and `None` otherwise.
pub fn value_to_f64(value: &Value) -> Option<f64> {
    if let Value::String(s) = value {
        s.parse::<f64>().map_err(|e| println!("{}", e)).ok()
    } else {
        None
    }
}

/// Converts a JSON `Value` to an `Option<i64>`.
/// 
/// # Arguments
/// 
/// * `value` - A reference to the JSON `Value`.
/// 
/// # Returns
/// 
/// An `Option<i64>` which is `Some` if the value can be parsed as a `i64`, and `None` otherwise.
pub fn value_to_i64(value: &Value) -> Option<i64> {
    if let Value::String(s) = value {
        s.parse::<i64>().map_err(|e| println!("{}", e)).ok()
    } else {
        None
    }
}

/// HTTP GET endpoint to retrieve JSON data for a specified host.
/// 
/// # Arguments
/// 
/// * `host` - A `web::Path<String>` representing the host.
/// * `state` - A `web::Data<AppState>` representing the application state.
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// 
/// # Returns
/// 
/// An `impl Responder` containing the JSON response.
#[get("/get_json/{host}")]
pub async fn get_json(
    host: web::Path<String>, 
    state: web::Data<AppState>, 
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>) 
-> impl Responder {
    // Lock the host dictionary for reading.
    let host_dictionary = host_dictionary.lock().unwrap();
    let host = host.clone();
    // Retrieve the host parameters.
    let host_parameters = host_dictionary.get(&host).unwrap().clone();
    // Define the payload for the NETCONF request.
    let payload: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="101"
     xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
  <get>
    <filter type="subtree">
      <components xmlns="http://openconfig.net/yang/platform">
        <component>
          <transceiver xmlns="http://openconfig.net/yang/platform/transceiver"/>
        </component>
      </components>
    </filter>
  </get>
</rpc>
"#;

    // Send the NETCONF request and convert the response to JSON.
    let json_response = to_json(
        &get(&host, host_parameters.port, &host_parameters.user, &host_parameters.password, payload).unwrap()
    ).unwrap();

    // Initialize a list to store component data.
    let mut components_list: Vec<HashMap<String, Value>> = vec![];

    // Process the JSON response to extract component data.
    if let Some(components) = json_response.pointer(
        "/rpc-reply/data/components/component"
    ).and_then(|c| c.as_array()) {
        // Define JSON values for "PRESENT" and "NOT_PRESENT".
        let present = json!("PRESENT");
        let not_present = json!("NOT_PRESENT");

        for component in components {
            let mut json_component: HashMap<String, Value> = HashMap::new();
            json_component.insert("name".to_string(), component.get("name").unwrap().clone());

            if component.pointer("/transceiver/state/present").unwrap().eq(&present) {
                json_component.insert("present-state".to_string(), present.clone());
                json_component.insert("serial-no".to_string(), component.pointer("/transceiver/state/serial-no").unwrap().clone());
                json_component.insert("vendor".to_string(), component.pointer("/transceiver/state/vendor").unwrap().clone());
                json_component.insert("vendor-part".to_string(), component.pointer("/transceiver/state/vendor-part").unwrap().clone());
                json_component.insert("vendor-rev".to_string(), component.pointer("/transceiver/state/vendor-rev").unwrap().clone());

                if let Some(physical_channels) = component.pointer("/transceiver/physical-channels") {
                    let mut channels_list: Vec<Value> = vec![];

                    match physical_channels.get("channel") {
                        Some(Value::Array(channels)) => {
                            // Handle the case where "channel" is an array
                            for channel in channels {
                                let mut json_channel: HashMap<String, Value> = HashMap::new();
                                if let Some(state) = channel.get("state") {
                                    for (k, v) in state.as_object().unwrap().clone() {
                                        json_channel.insert(k.clone(), v.clone());
                                    }
                                };
                                channels_list.push(json!(json_channel));
                            }
                        }
                        Some(Value::Object(channel)) => {
                            // Handle the case where "channel" is a single JSON object
                            let mut json_channel: HashMap<String, Value> = HashMap::new();
                            if let Some(state) = channel.get("state") {
                                for (k, v) in state.as_object().unwrap().clone() {
                                    json_channel.insert(k.clone(), v.clone());
                                }
                            };
                            channels_list.push(json!(json_channel));
                        }
                        _ => {}
                    }

                    if !channels_list.is_empty() {
                        json_component.insert("channel".to_string(), json!(channels_list));
                    }
                }
            } else {
                json_component.insert("present-state".to_string(), not_present.clone());
            }
            components_list.push(json_component);
        }
    }

    // Update the application state with metric data from the JSON response.
    for component in &components_list {
        let name = format!("{} : {}", &component.get("name").unwrap().clone().to_string(), &host);

        if component.get("present-state").unwrap().clone().to_string().eq(&"\"PRESENT\"".to_string()) {
            if let Some(channels) = component.get("channel").clone().and_then(|c| c.as_array()) {
                for channel in channels {
                    if let Some(input_power) = channel.get("input-power").clone() {
                        if let Some(input_power_avg) = input_power.get("avg").clone() {
                            state.optic_data.channel.input_power.avg
                                .with_label_values(&[&name])
                                .set(value_to_f64(input_power_avg).unwrap())
                        }
                        if let Some(input_power_instant) = input_power.get("instant").clone() {
                            state.optic_data.channel.input_power.instant
                                .with_label_values(&[&name])
                                .set(value_to_f64(input_power_instant).unwrap())
                        }
                        if let Some(input_power_interval) = input_power.get("interval").clone() {
                            state.optic_data.channel.input_power.interval
                                .with_label_values(&[&name])
                                .set(value_to_i64(input_power_interval).unwrap() as f64)
                        }
                        if let Some(input_power_max) = input_power.get("max").clone() {
                            state.optic_data.channel.input_power.max
                                .with_label_values(&[&name])
                                .set(value_to_f64(input_power_max).unwrap())
                        }
                        if let Some(input_power_max_time) = input_power.get("max-time").clone() {
                            state.optic_data.channel.input_power.max_time
                                .with_label_values(&[&name])
                                .set(value_to_i64(input_power_max_time).unwrap() as f64)
                        }
                        if let Some(input_power_min) = input_power.get("min").clone() {
                            state.optic_data.channel.input_power.min
                                .with_label_values(&[&name])
                                .set(value_to_f64(input_power_min).unwrap())
                        }
                        if let Some(input_power_min_time) = input_power.get("min-time").clone() {
                            state.optic_data.channel.input_power.min_time
                                .with_label_values(&[&name])
                                .set(value_to_i64(input_power_min_time).unwrap() as f64)
                        }
                    }

                    if let Some(laser_bias_current) = channel.get("laser-bias-current").clone() {
                        if let Some(laser_bias_current_avg) = laser_bias_current.get("avg").clone() {
                            state.optic_data.channel.laser_bias_current.avg
                                .with_label_values(&[&name])
                                .set(value_to_f64(laser_bias_current_avg).unwrap())
                        }
                        if let Some(laser_bias_current_instant) = laser_bias_current.get("instant").clone() {
                            state.optic_data.channel.laser_bias_current.instant
                                .with_label_values(&[&name])
                                .set(value_to_f64(laser_bias_current_instant).unwrap())
                        }
                        if let Some(laser_bias_current_interval) = laser_bias_current.get("interval").clone() {
                            state.optic_data.channel.laser_bias_current.interval
                                .with_label_values(&[&name])
                                .set(value_to_i64(laser_bias_current_interval).unwrap() as f64)
                        }
                        if let Some(laser_bias_current_max) = laser_bias_current.get("max").clone() {
                            state.optic_data.channel.laser_bias_current.max
                                .with_label_values(&[&name])
                                .set(value_to_f64(laser_bias_current_max).unwrap())
                        }
                        if let Some(laser_bias_current_max_time) = laser_bias_current.get("max-time").clone() {
                            state.optic_data.channel.laser_bias_current.max_time
                                .with_label_values(&[&name])
                                .set(value_to_i64(laser_bias_current_max_time).unwrap() as f64)
                        }
                        if let Some(laser_bias_current_min) = laser_bias_current.get("min").clone() {
                            state.optic_data.channel.laser_bias_current.min
                                .with_label_values(&[&name])
                                .set(value_to_f64(laser_bias_current_min).unwrap())
                        }
                        if let Some(laser_bias_current_min_time) = laser_bias_current.get("min-time").clone() {
                            state.optic_data.channel.laser_bias_current.min_time
                                .with_label_values(&[&name])
                                .set(value_to_i64(laser_bias_current_min_time).unwrap() as f64)
                        }
                    }

                    if let Some(output_power) = channel.get("output-power").clone() {
                        if let Some(output_power_avg) = output_power.get("avg").clone() {
                            state.optic_data.channel.output_power.avg
                                .with_label_values(&[&name])
                                .set(value_to_f64(output_power_avg).unwrap())
                        }
                        if let Some(output_power_instant) = output_power.get("instant").clone() {
                            state.optic_data.channel.output_power.instant
                                .with_label_values(&[&name])
                                .set(value_to_f64(output_power_instant).unwrap())
                        }
                        if let Some(output_power_interval) = output_power.get("interval").clone() {
                            state.optic_data.channel.output_power.interval
                                .with_label_values(&[&name])
                                .set(value_to_i64(output_power_interval).unwrap() as f64)
                        }
                        if let Some(output_power_max) = output_power.get("max").clone() {
                            state.optic_data.channel.output_power.max
                                .with_label_values(&[&name])
                                .set(value_to_f64(output_power_max).unwrap())
                        }
                        if let Some(output_power_max_time) = output_power.get("max-time").clone() {
                            state.optic_data.channel.output_power.max_time
                                .with_label_values(&[&name])
                                .set(value_to_i64(output_power_max_time).unwrap() as f64)
                        }
                        if let Some(output_power_min) = output_power.get("min").clone() {
                            state.optic_data.channel.output_power.min
                                .with_label_values(&[&name])
                                .set(value_to_f64(output_power_min).unwrap())
                        }
                        if let Some(output_power_min_time) = output_power.get("min-time").clone() {
                            state.optic_data.channel.output_power.min_time
                                .with_label_values(&[&name])
                                .set(value_to_i64(output_power_min_time).unwrap() as f64)
                        }
                    }
                }
            }
        }
    }
    // Return the JSON response.
    web::Json(components_list)
}
