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
pub struct ControllerConfigurationDto {
    /// The maximum number of timer driven threads the NiFi has available.
    #[serde(rename = "maxTimerDrivenThreadCount", skip_serializing_if = "Option::is_none")]
    pub max_timer_driven_thread_count: Option<i32>,
    /// The maximum number of event driven threads the NiFi has available.
    #[serde(rename = "maxEventDrivenThreadCount", skip_serializing_if = "Option::is_none")]
    pub max_event_driven_thread_count: Option<i32>,
}

impl ControllerConfigurationDto {
    pub fn new() -> ControllerConfigurationDto {
        ControllerConfigurationDto {
            max_timer_driven_thread_count: None,
            max_event_driven_thread_count: None,
        }
    }
}

