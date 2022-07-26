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
pub struct SystemDiagnosticsDto {
    #[serde(rename = "aggregateSnapshot", skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<Box<crate::models::SystemDiagnosticsSnapshotDto>>,
    /// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots", skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<crate::models::NodeSystemDiagnosticsSnapshotDto>>,
}

impl SystemDiagnosticsDto {
    pub fn new() -> SystemDiagnosticsDto {
        SystemDiagnosticsDto {
            aggregate_snapshot: None,
            node_snapshots: None,
        }
    }
}


