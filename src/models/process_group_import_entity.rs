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
pub struct ProcessGroupImportEntity {
    #[serde(rename = "processGroupRevision", skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<Box<crate::models::RevisionDto>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged", skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "versionedFlowSnapshot", skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<Box<crate::models::VersionedFlowSnapshot>>,
}

impl ProcessGroupImportEntity {
    pub fn new() -> ProcessGroupImportEntity {
        ProcessGroupImportEntity {
            process_group_revision: None,
            disconnected_node_acknowledged: None,
            versioned_flow_snapshot: None,
        }
    }
}

