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
pub struct PropertyAllowableValue {
    /// The internal value
    #[serde(rename = "value")]
    pub value: String,
    /// The display name of the value, if different from the internal value
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The description of the value, e.g., the behavior it produces.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PropertyAllowableValue {
    pub fn new(value: String) -> PropertyAllowableValue {
        PropertyAllowableValue {
            value,
            display_name: None,
            description: None,
        }
    }
}

