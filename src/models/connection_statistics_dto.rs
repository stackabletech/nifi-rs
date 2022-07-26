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
pub struct ConnectionStatisticsDto {
    /// The ID of the connection
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The timestamp of when the stats were last refreshed
    #[serde(rename = "statsLastRefreshed", skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    #[serde(rename = "aggregateSnapshot", skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<Box<crate::models::ConnectionStatisticsSnapshotDto>>,
    /// A list of status snapshots for each node
    #[serde(rename = "nodeSnapshots", skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<crate::models::NodeConnectionStatisticsSnapshotDto>>,
}

impl ConnectionStatisticsDto {
    pub fn new() -> ConnectionStatisticsDto {
        ConnectionStatisticsDto {
            id: None,
            stats_last_refreshed: None,
            aggregate_snapshot: None,
            node_snapshots: None,
        }
    }
}


