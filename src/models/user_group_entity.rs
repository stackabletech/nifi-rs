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
pub struct UserGroupEntity {
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Box<crate::models::RevisionDto>>,
    /// The id of the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI for futures requests to the component.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::PositionDto>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::PermissionsDto>>,
    /// The bulletins for this component.
    #[serde(rename = "bulletins", skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<crate::models::BulletinEntity>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged", skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<Box<crate::models::UserGroupDto>>,
}

impl UserGroupEntity {
    pub fn new() -> UserGroupEntity {
        UserGroupEntity {
            revision: None,
            id: None,
            uri: None,
            position: None,
            permissions: None,
            bulletins: None,
            disconnected_node_acknowledged: None,
            component: None,
        }
    }
}


