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
pub struct VersionedRemoteProcessGroup {
    /// The component's unique identifier
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier", skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user-supplied comments for the component
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::Position>>,
    /// [DEPRECATED] The target URI of the remote process group. If target uri is not set, but uris are set, then returns the first uri in the uris. If neither target uri nor uris are set, then returns null.
    #[serde(rename = "targetUri", skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The target URIs of the remote process group. If target uris is not set but target uri is set, then returns the single target uri. If neither target uris nor target uri is set, then returns null.
    #[serde(rename = "targetUris", skip_serializing_if = "Option::is_none")]
    pub target_uris: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    #[serde(rename = "communicationsTimeout", skip_serializing_if = "Option::is_none")]
    pub communications_timeout: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    #[serde(rename = "yieldDuration", skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
    /// The Transport Protocol that is used for Site-to-Site communications
    #[serde(rename = "transportProtocol", skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<TransportProtocol>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    #[serde(rename = "localNetworkInterface", skip_serializing_if = "Option::is_none")]
    pub local_network_interface: Option<String>,
    #[serde(rename = "proxyHost", skip_serializing_if = "Option::is_none")]
    pub proxy_host: Option<String>,
    #[serde(rename = "proxyPort", skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyUser", skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
    /// A Set of Input Ports that can be connected to, in order to send data to the remote NiFi instance
    #[serde(rename = "inputPorts", skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<crate::models::VersionedRemoteGroupPort>>,
    /// A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance
    #[serde(rename = "outputPorts", skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<crate::models::VersionedRemoteGroupPort>>,
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<ComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier", skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
}

impl VersionedRemoteProcessGroup {
    pub fn new() -> VersionedRemoteProcessGroup {
        VersionedRemoteProcessGroup {
            identifier: None,
            instance_identifier: None,
            name: None,
            comments: None,
            position: None,
            target_uri: None,
            target_uris: None,
            communications_timeout: None,
            yield_duration: None,
            transport_protocol: None,
            local_network_interface: None,
            proxy_host: None,
            proxy_port: None,
            proxy_user: None,
            input_ports: None,
            output_ports: None,
            component_type: None,
            group_identifier: None,
        }
    }
}

/// The Transport Protocol that is used for Site-to-Site communications
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransportProtocol {
    #[serde(rename = "RAW")]
    RAW,
    #[serde(rename = "HTTP")]
    HTTP,
}

impl Default for TransportProtocol {
    fn default() -> TransportProtocol {
        Self::RAW
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "CONNECTION")]
    CONNECTION,
    #[serde(rename = "PROCESSOR")]
    PROCESSOR,
    #[serde(rename = "PROCESS_GROUP")]
    PROCESSGROUP,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    REMOTEPROCESSGROUP,
    #[serde(rename = "INPUT_PORT")]
    INPUTPORT,
    #[serde(rename = "OUTPUT_PORT")]
    OUTPUTPORT,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    REMOTEINPUTPORT,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    REMOTEOUTPUTPORT,
    #[serde(rename = "FUNNEL")]
    FUNNEL,
    #[serde(rename = "LABEL")]
    LABEL,
    #[serde(rename = "CONTROLLER_SERVICE")]
    CONTROLLERSERVICE,
    #[serde(rename = "REPORTING_TASK")]
    REPORTINGTASK,
    #[serde(rename = "PARAMETER_CONTEXT")]
    PARAMETERCONTEXT,
    #[serde(rename = "TEMPLATE")]
    TEMPLATE,
}

impl Default for ComponentType {
    fn default() -> ComponentType {
        Self::CONNECTION
    }
}

