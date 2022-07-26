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
pub struct VersionedFlowDto {
    /// The ID of the registry that the flow is tracked to
    #[serde(rename = "registryId", skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The ID of the bucket where the flow is stored
    #[serde(rename = "bucketId", skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The ID of the flow
    #[serde(rename = "flowId", skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the flow
    #[serde(rename = "flowName", skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    /// A description of the flow
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Comments for the changeset
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The action being performed
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl VersionedFlowDto {
    pub fn new() -> VersionedFlowDto {
        VersionedFlowDto {
            registry_id: None,
            bucket_id: None,
            flow_id: None,
            flow_name: None,
            description: None,
            comments: None,
            action: None,
        }
    }
}

/// The action being performed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "COMMIT")]
    COMMIT,
    #[serde(rename = "FORCE_COMMIT")]
    FORCECOMMIT,
}

impl Default for Action {
    fn default() -> Action {
        Self::COMMIT
    }
}

