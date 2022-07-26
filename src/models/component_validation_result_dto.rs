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
pub struct ComponentValidationResultDto {
    /// The UUID of the Process Group that this component is in
    #[serde(rename = "processGroupId", skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The UUID of this component
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of this component
    #[serde(rename = "referenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ReferenceType>,
    /// The name of this component.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The number of active threads for the referencing component.
    #[serde(rename = "activeThreadCount", skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The validation errors for the component.
    #[serde(rename = "validationErrors", skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Whether or not the component is currently valid
    #[serde(rename = "currentlyValid", skip_serializing_if = "Option::is_none")]
    pub currently_valid: Option<bool>,
    /// Whether or not the component will be valid if the Parameter Context is changed
    #[serde(rename = "resultsValid", skip_serializing_if = "Option::is_none")]
    pub results_valid: Option<bool>,
    /// The validation errors that will apply to the component if the Parameter Context is changed
    #[serde(rename = "resultantValidationErrors", skip_serializing_if = "Option::is_none")]
    pub resultant_validation_errors: Option<Vec<String>>,
}

impl ComponentValidationResultDto {
    pub fn new() -> ComponentValidationResultDto {
        ComponentValidationResultDto {
            process_group_id: None,
            id: None,
            reference_type: None,
            name: None,
            state: None,
            active_thread_count: None,
            validation_errors: None,
            currently_valid: None,
            results_valid: None,
            resultant_validation_errors: None,
        }
    }
}

/// The type of this component
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReferenceType {
    #[serde(rename = "PROCESSOR")]
    PROCESSOR,
    #[serde(rename = "CONTROLLER_SERVICE")]
    CONTROLLERSERVICE,
    #[serde(rename = "INPUT_PORT")]
    INPUTPORT,
    #[serde(rename = "OUTPUT_PORT")]
    OUTPUTPORT,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    REMOTEINPUTPORT,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    REMOTEOUTPUTPORT,
}

impl Default for ReferenceType {
    fn default() -> ReferenceType {
        Self::PROCESSOR
    }
}
