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
pub struct ControllerServiceStatusDto {
    /// The run status of this ControllerService
    #[serde(rename = "runStatus", skip_serializing_if = "Option::is_none")]
    pub run_status: Option<RunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ValidationStatus>,
    /// The number of active threads for the component.
    #[serde(rename = "activeThreadCount", skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
}

impl ControllerServiceStatusDto {
    pub fn new() -> ControllerServiceStatusDto {
        ControllerServiceStatusDto {
            run_status: None,
            validation_status: None,
            active_thread_count: None,
        }
    }
}

/// The run status of this ControllerService
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RunStatus {
    #[serde(rename = "ENABLED")]
    ENABLED,
    #[serde(rename = "ENABLING")]
    ENABLING,
    #[serde(rename = "DISABLED")]
    DISABLED,
    #[serde(rename = "DISABLING")]
    DISABLING,
}

impl Default for RunStatus {
    fn default() -> RunStatus {
        Self::ENABLED
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
