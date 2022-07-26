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
pub struct VersionControlInformationDto {
    /// The ID of the Process Group that is under version control
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The ID of the registry that the flow is stored in
    #[serde(rename = "registryId", skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The name of the registry that the flow is stored in
    #[serde(rename = "registryName", skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// The ID of the bucket that the flow is stored in
    #[serde(rename = "bucketId", skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The name of the bucket that the flow is stored in
    #[serde(rename = "bucketName", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The ID of the flow
    #[serde(rename = "flowId", skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the flow
    #[serde(rename = "flowName", skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    /// The description of the flow
    #[serde(rename = "flowDescription", skip_serializing_if = "Option::is_none")]
    pub flow_description: Option<String>,
    /// The version of the flow
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The current state of the Process Group, as it relates to the Versioned Flow
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Explanation of why the group is in the specified state
    #[serde(rename = "stateExplanation", skip_serializing_if = "Option::is_none")]
    pub state_explanation: Option<String>,
}

impl VersionControlInformationDto {
    pub fn new() -> VersionControlInformationDto {
        VersionControlInformationDto {
            group_id: None,
            registry_id: None,
            registry_name: None,
            bucket_id: None,
            bucket_name: None,
            flow_id: None,
            flow_name: None,
            flow_description: None,
            version: None,
            state: None,
            state_explanation: None,
        }
    }
}

/// The current state of the Process Group, as it relates to the Versioned Flow
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "LOCALLY_MODIFIED")]
    LOCALLYMODIFIED,
    #[serde(rename = "STALE")]
    STALE,
    #[serde(rename = "LOCALLY_MODIFIED_AND_STALE")]
    LOCALLYMODIFIEDANDSTALE,
    #[serde(rename = "UP_TO_DATE")]
    UPTODATE,
    #[serde(rename = "SYNC_FAILURE")]
    SYNCFAILURE,
}

impl Default for State {
    fn default() -> State {
        Self::LOCALLYMODIFIED
    }
}

