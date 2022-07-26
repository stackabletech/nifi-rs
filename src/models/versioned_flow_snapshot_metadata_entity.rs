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
pub struct VersionedFlowSnapshotMetadataEntity {
    #[serde(rename = "versionedFlowSnapshotMetadata", skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot_metadata: Option<Box<crate::models::VersionedFlowSnapshotMetadata>>,
    /// The ID of the Registry that this flow belongs to
    #[serde(rename = "registryId", skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

impl VersionedFlowSnapshotMetadataEntity {
    pub fn new() -> VersionedFlowSnapshotMetadataEntity {
        VersionedFlowSnapshotMetadataEntity {
            versioned_flow_snapshot_metadata: None,
            registry_id: None,
        }
    }
}

