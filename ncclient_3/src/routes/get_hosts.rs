use actix_web::{get, web, Responder};
use std::collections::HashMap;
use serde_json::json;
use std::sync::{Arc, Mutex};

use crate::HostParameters;

/// HTTP GET endpoint to retrieve a list of all hosts.
/// 
/// # Arguments
/// 
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// 
/// # Returns
/// 
/// An `impl Responder` containing the JSON response with a list of host names.
#[get("/get_hosts")]
pub async fn get_hosts(
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>) 
-> impl Responder {
    // Lock the host dictionary for reading.
    let host_dictionary = host_dictionary.lock().unwrap();
    // Initialize a vector to store host names.
    let mut hosts_vector: Vec<&String> = vec![];
    // Collect all host names from the dictionary.
    for host in host_dictionary.keys() {
        hosts_vector.push(host);
    }
    // Convert the host names vector to JSON.
    let host_vector_json = json!(hosts_vector);
    // Return the JSON response.
    web::Json(host_vector_json)
}
