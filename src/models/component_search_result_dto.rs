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
pub struct ComponentSearchResultDto {
    /// The id of the component that matched the search.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The group id of the component that matched the search.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "parentGroup", skip_serializing_if = "Option::is_none")]
    pub parent_group: Option<Box<crate::models::SearchResultGroupDto>>,
    #[serde(rename = "versionedGroup", skip_serializing_if = "Option::is_none")]
    pub versioned_group: Option<Box<crate::models::SearchResultGroupDto>>,
    /// The name of the component that matched the search.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What matched the search from the component.
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
}

impl ComponentSearchResultDto {
    pub fn new() -> ComponentSearchResultDto {
        ComponentSearchResultDto {
            id: None,
            group_id: None,
            parent_group: None,
            versioned_group: None,
            name: None,
            matches: None,
        }
    }
}

