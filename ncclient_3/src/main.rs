mod netconf;
mod utils;
mod routes;
mod models;
mod opt_utils;

use routes::{
    get_json::get_json,
    add_host::add_host,
    get_hosts::get_hosts,
};
use models::ComponentMetrics;
use opt_utils::register_init_metric;

use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// State structure for the Actix web application.
#[derive(Clone, Debug)]
pub struct AppState {
    /// Metric data for optical components.
    optic_data: ComponentMetrics,
}

/// Structure to hold host parameters including port, user, and password.
#[derive(Clone, Debug)]
pub struct HostParameters {
    /// The port number for the host.
    port: isize,
    /// The username for the host.
    user: String,
    /// The password for the host.
    password: String,
}

/// Main entry point for the Actix web application.
/// 
/// This function sets up the HTTP server, initializes application state, 
/// configures Prometheus metrics, and defines the application routes.
/// 
/// # Returns
/// 
/// A `Result` indicating success or failure of the server setup.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a thread-safe dictionary to store host parameters.
    let host_dictionary: Arc<Mutex<HashMap<String, HostParameters>>> = Arc::new(Mutex::new(HashMap::new()));
    // Create a new Prometheus registry.
    let registry = prometheus::Registry::new();

    // Initialize the application state with metrics.
    let app_state = register_init_metric(registry.clone());

    // Configure Prometheus metrics.
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .registry(registry)
        .build()
        .unwrap();

    // Configure and run the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(host_dictionary.clone()))
            .wrap(prometheus.clone())
            .service(get_json)
            .service(add_host)
            .service(get_hosts)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
