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
pub struct ReportingTaskRunStatusEntity {
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Box<crate::models::RevisionDto>>,
    /// The run status of the ReportingTask.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged", skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
}

impl ReportingTaskRunStatusEntity {
    pub fn new() -> ReportingTaskRunStatusEntity {
        ReportingTaskRunStatusEntity {
            revision: None,
            state: None,
            disconnected_node_acknowledged: None,
        }
    }
}

/// The run status of the ReportingTask.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "STOPPED")]
    STOPPED,
}

impl Default for State {
    fn default() -> State {
        Self::RUNNING
    }
}

