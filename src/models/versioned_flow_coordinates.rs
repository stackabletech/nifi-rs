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
pub struct VersionedFlowCoordinates {
    /// The URL of the Flow Registry that contains the flow
    #[serde(rename = "registryUrl", skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    /// The UUID of the bucket that the flow resides in
    #[serde(rename = "bucketId", skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The UUID of the flow
    #[serde(rename = "flowId", skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The version of the flow
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Whether or not these coordinates point to the latest version of the flow
    #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
}

impl VersionedFlowCoordinates {
    pub fn new() -> VersionedFlowCoordinates {
        VersionedFlowCoordinates {
            registry_url: None,
            bucket_id: None,
            flow_id: None,
            version: None,
            latest: None,
        }
    }
}


