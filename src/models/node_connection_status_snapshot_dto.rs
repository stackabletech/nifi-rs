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
pub struct NodeConnectionStatusSnapshotDto {
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// The API address of the node
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort", skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    #[serde(rename = "statusSnapshot", skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<Box<crate::models::ConnectionStatusSnapshotDto>>,
}

impl NodeConnectionStatusSnapshotDto {
    pub fn new() -> NodeConnectionStatusSnapshotDto {
        NodeConnectionStatusSnapshotDto {
            node_id: None,
            address: None,
            api_port: None,
            status_snapshot: None,
        }
    }
}


