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
pub struct PreviousValueDto {
    /// The previous value.
    #[serde(rename = "previousValue", skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
    /// The timestamp when the value was modified.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The user who changed the previous value.
    #[serde(rename = "userIdentity", skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
}

impl PreviousValueDto {
    pub fn new() -> PreviousValueDto {
        PreviousValueDto {
            previous_value: None,
            timestamp: None,
            user_identity: None,
        }
    }
}

