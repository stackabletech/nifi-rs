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
pub struct ProcessGroupStatusEntity {
    #[serde(rename = "processGroupStatus", skip_serializing_if = "Option::is_none")]
    pub process_group_status: Option<Box<crate::models::ProcessGroupStatusDto>>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead", skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
}

impl ProcessGroupStatusEntity {
    pub fn new() -> ProcessGroupStatusEntity {
        ProcessGroupStatusEntity {
            process_group_status: None,
            can_read: None,
        }
    }
}


