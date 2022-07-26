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
pub struct UserDto {
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
    /// The identity of the tenant.
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Whether this tenant is configurable.
    #[serde(rename = "configurable", skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The groups to which the user belongs. This field is read only and it provided for convenience.
    #[serde(rename = "userGroups", skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<crate::models::TenantEntity>>,
    /// The access policies this user belongs to.
    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Vec<crate::models::AccessPolicySummaryEntity>>,
}

impl UserDto {
    pub fn new() -> UserDto {
        UserDto {
            id: None,
            versioned_component_id: None,
            parent_group_id: None,
            position: None,
            identity: None,
            configurable: None,
            user_groups: None,
            access_policies: None,
        }
    }
}

