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
pub struct ConnectionStatusSnapshotEntity {
    /// The id of the connection.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "connectionStatusSnapshot", skip_serializing_if = "Option::is_none")]
    pub connection_status_snapshot: Option<Box<crate::models::ConnectionStatusSnapshotDto>>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead", skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
}

impl ConnectionStatusSnapshotEntity {
    pub fn new() -> ConnectionStatusSnapshotEntity {
        ConnectionStatusSnapshotEntity {
            id: None,
            connection_status_snapshot: None,
            can_read: None,
        }
    }
}


