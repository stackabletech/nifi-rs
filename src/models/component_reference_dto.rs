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
pub struct ComponentReferenceDto {
    /// The id of the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId", skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId", skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::PositionDto>>,
    /// The name of the component.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ComponentReferenceDto {
    pub fn new() -> ComponentReferenceDto {
        ComponentReferenceDto {
            id: None,
            versioned_component_id: None,
            parent_group_id: None,
            position: None,
            name: None,
        }
    }
}

