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
pub struct StatusDescriptorDto {
    /// The name of the status field.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The label for the status field.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The description of the status field.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The formatter for the status descriptor.
    #[serde(rename = "formatter", skip_serializing_if = "Option::is_none")]
    pub formatter: Option<String>,
}

impl StatusDescriptorDto {
    pub fn new() -> StatusDescriptorDto {
        StatusDescriptorDto {
            field: None,
            label: None,
            description: None,
            formatter: None,
        }
    }
}

