use actix_web::{post, web, Responder, HttpResponse};
use std::collections::HashMap;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::HostParameters;

/// Struct representing the request body for adding a new host.
#[derive(Debug, Clone, Deserialize)]
struct AddHostRequest {
    host: String,
    port: isize,
    user: String,
    password: String,
}

/// HTTP POST endpoint to add a new host to the host dictionary.
/// 
/// # Arguments
/// 
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// * `request` - A `web::Json<AddHostRequest>` representing the request body containing host details.
/// 
/// # Returns
/// 
/// An `impl Responder` containing an `HttpResponse` indicating the result of the operation.
#[post("/add_host")]
pub async fn add_host(
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>, 
    request: web::Json<AddHostRequest>
) -> impl Responder {
    // Lock the host dictionary for writing.
    let mut host_dictionary = host_dictionary.lock().unwrap();
    // Create a new HostParameters instance from the request data.
    let host_parameters = HostParameters {
        port: request.port.clone(),
        user: request.user.clone(),
        password: request.password.clone(),
    };
    // Insert the new host into the dictionary.
    host_dictionary.insert(request.host.clone(), host_parameters);
    // Return an HTTP response indicating successful addition.
    HttpResponse::Ok().message_body(format!("{} added successfully", request.host.clone()))
}

