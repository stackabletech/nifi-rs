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
pub struct BulletinEntity {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "sourceId", skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// When this bulletin was generated.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "nodeAddress", skip_serializing_if = "Option::is_none")]
    pub node_address: Option<String>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead", skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "bulletin", skip_serializing_if = "Option::is_none")]
    pub bulletin: Option<Box<crate::models::BulletinDto>>,
}

impl BulletinEntity {
    pub fn new() -> BulletinEntity {
        BulletinEntity {
            id: None,
            group_id: None,
            source_id: None,
            timestamp: None,
            node_address: None,
            can_read: None,
            bulletin: None,
        }
    }
}


