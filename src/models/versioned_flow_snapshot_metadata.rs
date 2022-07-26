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
pub struct VersionedFlowSnapshotMetadata {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<crate::models::JaxbLink>>,
    /// The identifier of the bucket this snapshot belongs to.
    #[serde(rename = "bucketIdentifier")]
    pub bucket_identifier: String,
    /// The identifier of the flow this snapshot belongs to.
    #[serde(rename = "flowIdentifier")]
    pub flow_identifier: String,
    /// The version of this snapshot of the flow.
    #[serde(rename = "version")]
    pub version: i32,
    /// The timestamp when the flow was saved, as milliseconds since epoch.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The user that created this snapshot of the flow.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// The comments provided by the user when creating the snapshot.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl VersionedFlowSnapshotMetadata {
    pub fn new(bucket_identifier: String, flow_identifier: String, version: i32) -> VersionedFlowSnapshotMetadata {
        VersionedFlowSnapshotMetadata {
            link: None,
            bucket_identifier,
            flow_identifier,
            version,
            timestamp: None,
            author: None,
            comments: None,
        }
    }
}

