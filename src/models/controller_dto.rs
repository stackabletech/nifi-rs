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
pub struct ControllerDto {
    /// The id of the NiFi.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the NiFi.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The comments for the NiFi.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of running components in the NiFi.
    #[serde(rename = "runningCount", skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stopped components in the NiFi.
    #[serde(rename = "stoppedCount", skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of invalid components in the NiFi.
    #[serde(rename = "invalidCount", skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of disabled components in the NiFi.
    #[serde(rename = "disabledCount", skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The number of active remote ports contained in the NiFi.
    #[serde(rename = "activeRemotePortCount", skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The number of inactive remote ports contained in the NiFi.
    #[serde(rename = "inactiveRemotePortCount", skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports contained in the NiFi.
    #[serde(rename = "inputPortCount", skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The number of output ports in the NiFi.
    #[serde(rename = "outputPortCount", skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    /// The Socket Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    #[serde(rename = "remoteSiteListeningPort", skip_serializing_if = "Option::is_none")]
    pub remote_site_listening_port: Option<i32>,
    /// The HTTP(S) Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    #[serde(rename = "remoteSiteHttpListeningPort", skip_serializing_if = "Option::is_none")]
    pub remote_site_http_listening_port: Option<i32>,
    /// Indicates whether or not Site-to-Site communications with this instance is secure (2-way authentication).
    #[serde(rename = "siteToSiteSecure", skip_serializing_if = "Option::is_none")]
    pub site_to_site_secure: Option<bool>,
    /// If clustered, the id of the Cluster Manager, otherwise the id of the NiFi.
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// The input ports available to send data to for the NiFi.
    #[serde(rename = "inputPorts", skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<crate::models::PortDto>>,
    /// The output ports available to received data from the NiFi.
    #[serde(rename = "outputPorts", skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<crate::models::PortDto>>,
}

impl ControllerDto {
    pub fn new() -> ControllerDto {
        ControllerDto {
            id: None,
            name: None,
            comments: None,
            running_count: None,
            stopped_count: None,
            invalid_count: None,
            disabled_count: None,
            active_remote_port_count: None,
            inactive_remote_port_count: None,
            input_port_count: None,
            output_port_count: None,
            remote_site_listening_port: None,
            remote_site_http_listening_port: None,
            site_to_site_secure: None,
            instance_id: None,
            input_ports: None,
            output_ports: None,
        }
    }
}


