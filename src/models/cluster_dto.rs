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
pub struct ClusterDto {
    /// The collection of nodes that are part of the cluster.
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<crate::models::NodeDto>>,
    /// The timestamp the report was generated.
    #[serde(rename = "generated", skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
}

impl ClusterDto {
    pub fn new() -> ClusterDto {
        ClusterDto {
            nodes: None,
            generated: None,
        }
    }
}


