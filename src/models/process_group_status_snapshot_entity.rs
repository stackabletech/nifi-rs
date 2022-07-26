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
pub struct ProcessGroupStatusSnapshotEntity {
    /// The id of the process group.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "processGroupStatusSnapshot", skip_serializing_if = "Option::is_none")]
    pub process_group_status_snapshot: Option<Box<crate::models::ProcessGroupStatusSnapshotDto>>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead", skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
}

impl ProcessGroupStatusSnapshotEntity {
    pub fn new() -> ProcessGroupStatusSnapshotEntity {
        ProcessGroupStatusSnapshotEntity {
            id: None,
            process_group_status_snapshot: None,
            can_read: None,
        }
    }
}


