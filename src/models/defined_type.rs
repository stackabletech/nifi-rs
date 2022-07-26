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
pub struct DefinedType {
    /// The group name of the bundle that provides the referenced type.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub _type: String,
    /// The description of the type.
    #[serde(rename = "typeDescription", skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
}

impl DefinedType {
    pub fn new(_type: String) -> DefinedType {
        DefinedType {
            group: None,
            artifact: None,
            version: None,
            _type,
            type_description: None,
        }
    }
}


