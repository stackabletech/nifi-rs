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
pub struct ReportingTaskStatusDto {
    /// The run status of this ReportingTask
    #[serde(rename = "runStatus", skip_serializing_if = "Option::is_none")]
    pub run_status: Option<RunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ValidationStatus>,
    /// The number of active threads for the component.
    #[serde(rename = "activeThreadCount", skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
}

impl ReportingTaskStatusDto {
    pub fn new() -> ReportingTaskStatusDto {
        ReportingTaskStatusDto {
            run_status: None,
            validation_status: None,
            active_thread_count: None,
        }
    }
}

/// The run status of this ReportingTask
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RunStatus {
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "STOPPED")]
    STOPPED,
    #[serde(rename = "DISABLED")]
    DISABLED,
}

impl Default for RunStatus {
    fn default() -> RunStatus {
        Self::RUNNING
    }
}
/// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValidationStatus {
    #[serde(rename = "VALID")]
    VALID,
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "VALIDATING")]
    VALIDATING,
}

impl Default for ValidationStatus {
    fn default() -> ValidationStatus {
        Self::VALID
    }
}

