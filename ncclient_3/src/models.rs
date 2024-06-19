use prometheus::GaugeVec;

/// Represents a set of advanced metrics for a particular measurement type.
#[derive(Debug, Clone)]
pub struct AdvanceMetric {
    /// The average value metric.
    pub avg: GaugeVec,
    /// The instant value metric.
    pub instant: GaugeVec,
    /// The interval value metric.
    pub interval: GaugeVec,
    /// The maximum value metric.
    pub max: GaugeVec,
    /// The maximum time value metric.
    pub max_time: GaugeVec,
    /// The minimum value metric.
    pub min: GaugeVec,
    /// The minimum time value metric.
    pub min_time: GaugeVec,
}

/// Represents metrics for a specific channel, including input power, laser bias current, and output power.
#[derive(Debug, Clone)]
pub struct ChannelMetrics {
    /// Metrics related to input power.
    pub input_power: AdvanceMetric,
    /// Metrics related to laser bias current.
    pub laser_bias_current: AdvanceMetric,
    /// Metrics related to output power.
    pub output_power: AdvanceMetric,
}

/// Represents the component metrics, which include channel metrics.
#[derive(Debug, Clone)]
pub struct ComponentMetrics {
    /// Channel-specific metrics.
    pub channel: ChannelMetrics,
}

