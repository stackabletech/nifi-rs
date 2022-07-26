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
pub struct RelationshipDto {
    /// The relationship name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The relationship description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not flowfiles sent to this relationship should auto terminate.
    #[serde(rename = "autoTerminate", skip_serializing_if = "Option::is_none")]
    pub auto_terminate: Option<bool>,
    /// Whether or not flowfiles sent to this relationship should retry.
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: Option<bool>,
}

impl RelationshipDto {
    pub fn new() -> RelationshipDto {
        RelationshipDto {
            name: None,
            description: None,
            auto_terminate: None,
            retry: None,
        }
    }
}


