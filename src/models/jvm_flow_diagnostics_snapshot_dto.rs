/*
 * NiFi Rest API
 *
 * The Rest API provides programmatic access to command and control a NiFi instance in real time. Start and                                             stop processors, monitor queues, query provenance data, and more. Each endpoint below includes a description,                                             definitions of the expected input and output, potential response codes, and the authorizations required                                             to invoke each service.
 *
 * The version of the OpenAPI document: 1.16.0
 * Contact: dev@nifi.apache.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JvmFlowDiagnosticsSnapshotDto {
    /// How long this node has been running, formatted as hours:minutes:seconds.milliseconds
    #[serde(rename = "uptime", skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    /// The name of the Time Zone that is configured, if available
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// The number of timer-driven threads that are active
    #[serde(rename = "activeTimerDrivenThreads", skip_serializing_if = "Option::is_none")]
    pub active_timer_driven_threads: Option<i32>,
    /// The number of event-driven threads that are active
    #[serde(rename = "activeEventDrivenThreads", skip_serializing_if = "Option::is_none")]
    pub active_event_driven_threads: Option<i32>,
    /// The NiFi Bundles (NARs) that are loaded by NiFi
    #[serde(rename = "bundlesLoaded", skip_serializing_if = "Option::is_none")]
    pub bundles_loaded: Option<Vec<crate::models::BundleDto>>,
}

impl JvmFlowDiagnosticsSnapshotDto {
    pub fn new() -> JvmFlowDiagnosticsSnapshotDto {
        JvmFlowDiagnosticsSnapshotDto {
            uptime: None,
            time_zone: None,
            active_timer_driven_threads: None,
            active_event_driven_threads: None,
            bundles_loaded: None,
        }
    }
}


