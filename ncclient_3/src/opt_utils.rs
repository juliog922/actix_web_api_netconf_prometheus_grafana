use prometheus::{GaugeVec, Registry};

use crate::AppState;
use crate::models::{ComponentMetrics, ChannelMetrics, AdvanceMetric};

/// Creates a `GaugeVec` metric and registers it with the provided registry.
///
/// # Arguments
///
/// * `name` - The name of the metric.
/// * `help` - A help description of the metric.
/// * `labelnames` - A string representing the label names for the metric.
/// * `registry` - The `Registry` instance to register the metric.
///
/// # Returns
///
/// A `GaugeVec` metric instance.
fn return_opt(name: &str, help: &str, labelnames: &str, registry: Registry) -> GaugeVec {
    let state = GaugeVec::new(
        prometheus::Opts::new(name, help),
        &[labelnames],
    )
    .unwrap();
    registry.register(Box::new(state.clone())).unwrap();
    state
}

/// Registers and initializes metrics for the application.
///
/// # Arguments
///
/// * `registry` - The `Registry` instance to register the metrics.
///
/// # Returns
///
/// An `AppState` instance with initialized metrics.
pub fn register_init_metric(registry: Registry) -> AppState {
    AppState {
        optic_data: ComponentMetrics {
            channel: ChannelMetrics {
                input_power: AdvanceMetric {
                    avg: return_opt(
                        "input_power_avg",
                        "Input Power Average",
                        "input_power_avg",
                        registry.clone(),
                    )
                    .clone(),
                    instant: return_opt(
                        "input_power_instant",
                        "Input Power Instant",
                        "input_power_instant",
                        registry.clone(),
                    )
                    .clone(),
                    interval: return_opt(
                        "input_power_interval",
                        "Input Power Interval",
                        "input_power_interval",
                        registry.clone(),
                    )
                    .clone(),
                    max: return_opt(
                        "input_power_max",
                        "Input Power Max",
                        "input_power_max",
                        registry.clone(),
                    )
                    .clone(),
                    max_time: return_opt(
                        "input_power_max_time",
                        "Input Power Max Time",
                        "input_power_max_time",
                        registry.clone(),
                    )
                    .clone(),
                    min: return_opt(
                        "input_power_min",
                        "Input Power Min",
                        "input_power_min",
                        registry.clone(),
                    )
                    .clone(),
                    min_time: return_opt(
                        "input_power_min_time",
                        "Input Power Min Time",
                        "input_power_min_time",
                        registry.clone(),
                    )
                    .clone(),
                },
                laser_bias_current: AdvanceMetric {
                    avg: return_opt(
                        "laser_bias_current_avg",
                        "Laser Bias Current Average",
                        "laser_bias_current_avg",
                        registry.clone(),
                    )
                    .clone(),
                    instant: return_opt(
                        "laser_bias_current_instant",
                        "Laser Bias Current Instant",
                        "laser_bias_current_instant",
                        registry.clone(),
                    )
                    .clone(),
                    interval: return_opt(
                        "laser_bias_current_interval",
                        "Laser Bias Current Interval",
                        "laser_bias_current_interval",
                        registry.clone(),
                    )
                    .clone(),
                    max: return_opt(
                        "laser_bias_current_max",
                        "Laser Bias Current Max",
                        "laser_bias_current_max",
                        registry.clone(),
                    )
                    .clone(),
                    max_time: return_opt(
                        "laser_bias_current_max_time",
                        "Laser Bias Current Max Time",
                        "laser_bias_current_max_time",
                        registry.clone(),
                    )
                    .clone(),
                    min: return_opt(
                        "laser_bias_current_min",
                        "Laser Bias Current Min",
                        "laser_bias_current_min",
                        registry.clone(),
                    )
                    .clone(),
                    min_time: return_opt(
                        "laser_bias_current_min_time",
                        "Laser Bias Current Min Time",
                        "laser_bias_current_min_time",
                        registry.clone(),
                    )
                    .clone(),
                },
                output_power: AdvanceMetric {
                    avg: return_opt(
                        "output_power_avg",
                        "Output Power Average",
                        "output_power_avg",
                        registry.clone(),
                    )
                    .clone(),
                    instant: return_opt(
                        "output_power_instant",
                        "Output Power Instant",
                        "output_power_instant",
                        registry.clone(),
                    )
                    .clone(),
                    interval: return_opt(
                        "output_power_interval",
                        "Output Power Interval",
                        "output_power_interval",
                        registry.clone(),
                    )
                    .clone(),
                    max: return_opt(
                        "output_power_max",
                        "Output Power Max",
                        "output_power_max",
                        registry.clone(),
                    )
                    .clone(),
                    max_time: return_opt(
                        "output_power_max_time",
                        "Output Power Max Time",
                        "output_power_max_time",
                        registry.clone(),
                    )
                    .clone(),
                    min: return_opt(
                        "output_power_min",
                        "Output Power Min",
                        "output_power_min",
                        registry.clone(),
                    )
                    .clone(),
                    min_time: return_opt(
                        "output_power_min_time",
                        "Output Power Min Time",
                        "output_power_min_time",
                        registry.clone(),
                    )
                    .clone(),
                },
            },
        },
    }
}
