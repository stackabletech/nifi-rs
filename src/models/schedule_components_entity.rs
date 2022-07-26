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
pub struct ScheduleComponentsEntity {
    /// The id of the ProcessGroup
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The desired state of the descendant components
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Optional components to schedule. If not specified, all authorized descendant components will be used.
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<::std::collections::HashMap<String, crate::models::RevisionDto>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged", skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
}

impl ScheduleComponentsEntity {
    pub fn new() -> ScheduleComponentsEntity {
        ScheduleComponentsEntity {
            id: None,
            state: None,
            components: None,
            disconnected_node_acknowledged: None,
        }
    }
}

/// The desired state of the descendant components
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "STOPPED")]
    STOPPED,
    #[serde(rename = "ENABLED")]
    ENABLED,
    #[serde(rename = "DISABLED")]
    DISABLED,
}

impl Default for State {
    fn default() -> State {
        Self::RUNNING
    }
}
