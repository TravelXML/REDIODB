// src/monitoring.rs
//
// Provides Prometheus metrics integration.
use prometheus::{Encoder, TextEncoder, Counter, register_counter};
use lazy_static::lazy_static;

lazy_static! {
    // Counter for tracking the total number of processed commands.
    pub static ref COMMAND_COUNTER: Counter = register_counter!(
        "edgedb_command_total",
        "Total number of commands processed"
    ).unwrap();
}

/// Gathers and returns metrics in Prometheus text format.
pub fn get_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
