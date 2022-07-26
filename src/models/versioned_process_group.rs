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
pub struct VersionedProcessGroup {
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
    /// The child Process Groups
    #[serde(rename = "processGroups", skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<crate::models::VersionedProcessGroup>>,
    /// The Remote Process Groups
    #[serde(rename = "remoteProcessGroups", skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<crate::models::VersionedRemoteProcessGroup>>,
    /// The Processors
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::models::VersionedProcessor>>,
    /// The Input Ports
    #[serde(rename = "inputPorts", skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<crate::models::VersionedPort>>,
    /// The Output Ports
    #[serde(rename = "outputPorts", skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<crate::models::VersionedPort>>,
    /// The Connections
    #[serde(rename = "connections", skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<crate::models::VersionedConnection>>,
    /// The Labels
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::VersionedLabel>>,
    /// The Funnels
    #[serde(rename = "funnels", skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<crate::models::VersionedFunnel>>,
    /// The Controller Services
    #[serde(rename = "controllerServices", skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<crate::models::VersionedControllerService>>,
    #[serde(rename = "versionedFlowCoordinates", skip_serializing_if = "Option::is_none")]
    pub versioned_flow_coordinates: Option<Box<crate::models::VersionedFlowCoordinates>>,
    /// The Variables in the Variable Registry for this Process Group (not including any ancestor or descendant Process Groups)
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
    /// The name of the parameter context used by this process group
    #[serde(rename = "parameterContextName", skip_serializing_if = "Option::is_none")]
    pub parameter_context_name: Option<String>,
    /// The default FlowFile Expiration for this Process Group.
    #[serde(rename = "defaultFlowFileExpiration", skip_serializing_if = "Option::is_none")]
    pub default_flow_file_expiration: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureObjectThreshold", skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureDataSizeThreshold", skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// The configured FlowFile Concurrency for the Process Group
    #[serde(rename = "flowFileConcurrency", skip_serializing_if = "Option::is_none")]
    pub flow_file_concurrency: Option<String>,
    /// The FlowFile Outbound Policy for the Process Group
    #[serde(rename = "flowFileOutboundPolicy", skip_serializing_if = "Option::is_none")]
    pub flow_file_outbound_policy: Option<String>,
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<ComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier", skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
}

impl VersionedProcessGroup {
    pub fn new() -> VersionedProcessGroup {
        VersionedProcessGroup {
            identifier: None,
            instance_identifier: None,
            name: None,
            comments: None,
            position: None,
            process_groups: None,
            remote_process_groups: None,
            processors: None,
            input_ports: None,
            output_ports: None,
            connections: None,
            labels: None,
            funnels: None,
            controller_services: None,
            versioned_flow_coordinates: None,
            variables: None,
            parameter_context_name: None,
            default_flow_file_expiration: None,
            default_back_pressure_object_threshold: None,
            default_back_pressure_data_size_threshold: None,
            flow_file_concurrency: None,
            flow_file_outbound_policy: None,
            component_type: None,
            group_identifier: None,
        }
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

