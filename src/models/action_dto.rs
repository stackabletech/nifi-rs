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
pub struct ActionDto {
    /// The action id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The identity of the user that performed the action.
    #[serde(rename = "userIdentity", skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
    /// The timestamp of the action.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The id of the source component.
    #[serde(rename = "sourceId", skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component.
    #[serde(rename = "sourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The type of the source component.
    #[serde(rename = "sourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "componentDetails", skip_serializing_if = "Option::is_none")]
    pub component_details: Option<serde_json::Value>,
    /// The operation that was performed.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "actionDetails", skip_serializing_if = "Option::is_none")]
    pub action_details: Option<serde_json::Value>,
}

impl ActionDto {
    pub fn new() -> ActionDto {
        ActionDto {
            id: None,
            user_identity: None,
            timestamp: None,
            source_id: None,
            source_name: None,
            source_type: None,
            component_details: None,
            operation: None,
            action_details: None,
        }
    }
}


