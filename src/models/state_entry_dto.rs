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
pub struct StateEntryDto {
    /// The key for this state.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value for this state.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The identifier for the node where the state originated.
    #[serde(rename = "clusterNodeId", skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The label for the node where the state originated.
    #[serde(rename = "clusterNodeAddress", skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
}

impl StateEntryDto {
    pub fn new() -> StateEntryDto {
        StateEntryDto {
            key: None,
            value: None,
            cluster_node_id: None,
            cluster_node_address: None,
        }
    }
}

