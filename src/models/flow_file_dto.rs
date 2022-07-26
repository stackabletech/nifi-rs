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
pub struct FlowFileDto {
    /// The URI that can be used to access this FlowFile.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The FlowFile UUID.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// The FlowFile filename.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The FlowFile's position in the queue.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// The FlowFile file size.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// How long this FlowFile has been enqueued.
    #[serde(rename = "queuedDuration", skip_serializing_if = "Option::is_none")]
    pub queued_duration: Option<i64>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    #[serde(rename = "lineageDuration", skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// How long in milliseconds until the FlowFile penalty expires.
    #[serde(rename = "penaltyExpiresIn", skip_serializing_if = "Option::is_none")]
    pub penalty_expires_in: Option<i64>,
    /// The id of the node where this FlowFile resides.
    #[serde(rename = "clusterNodeId", skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The label for the node where this FlowFile resides.
    #[serde(rename = "clusterNodeAddress", skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The FlowFile attributes.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// The section in which the content claim lives.
    #[serde(rename = "contentClaimSection", skip_serializing_if = "Option::is_none")]
    pub content_claim_section: Option<String>,
    /// The container in which the content claim lives.
    #[serde(rename = "contentClaimContainer", skip_serializing_if = "Option::is_none")]
    pub content_claim_container: Option<String>,
    /// The identifier of the content claim.
    #[serde(rename = "contentClaimIdentifier", skip_serializing_if = "Option::is_none")]
    pub content_claim_identifier: Option<String>,
    /// The offset into the content claim where the flowfile's content begins.
    #[serde(rename = "contentClaimOffset", skip_serializing_if = "Option::is_none")]
    pub content_claim_offset: Option<i64>,
    /// The file size of the content claim formatted.
    #[serde(rename = "contentClaimFileSize", skip_serializing_if = "Option::is_none")]
    pub content_claim_file_size: Option<String>,
    /// The file size of the content claim in bytes.
    #[serde(rename = "contentClaimFileSizeBytes", skip_serializing_if = "Option::is_none")]
    pub content_claim_file_size_bytes: Option<i64>,
    /// If the FlowFile is penalized.
    #[serde(rename = "penalized", skip_serializing_if = "Option::is_none")]
    pub penalized: Option<bool>,
}

impl FlowFileDto {
    pub fn new() -> FlowFileDto {
        FlowFileDto {
            uri: None,
            uuid: None,
            filename: None,
            position: None,
            size: None,
            queued_duration: None,
            lineage_duration: None,
            penalty_expires_in: None,
            cluster_node_id: None,
            cluster_node_address: None,
            attributes: None,
            content_claim_section: None,
            content_claim_container: None,
            content_claim_identifier: None,
            content_claim_offset: None,
            content_claim_file_size: None,
            content_claim_file_size_bytes: None,
            penalized: None,
        }
    }
}


