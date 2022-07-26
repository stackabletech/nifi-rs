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
pub struct ProcessGroupDto {
    /// The id of the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId", skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId", skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::PositionDto>>,
    /// The name of the process group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The comments for the process group.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The variables that are configured for the Process Group. Note that this map contains only those variables that are defined on this Process Group and not any variables that are defined in the parent Process Group, etc. I.e., this Map will not contain all variables that are accessible by components in this Process Group by rather only the variables that are defined for this Process Group itself.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "versionControlInformation", skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<Box<crate::models::VersionControlInformationDto>>,
    #[serde(rename = "parameterContext", skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<Box<crate::models::ParameterContextReferenceEntity>>,
    /// The FlowFile Concurrency for this Process Group.
    #[serde(rename = "flowfileConcurrency", skip_serializing_if = "Option::is_none")]
    pub flowfile_concurrency: Option<FlowfileConcurrency>,
    /// The Outbound Policy that is used for determining how FlowFiles should be transferred out of the Process Group.
    #[serde(rename = "flowfileOutboundPolicy", skip_serializing_if = "Option::is_none")]
    pub flowfile_outbound_policy: Option<FlowfileOutboundPolicy>,
    /// The default FlowFile Expiration for this Process Group.
    #[serde(rename = "defaultFlowFileExpiration", skip_serializing_if = "Option::is_none")]
    pub default_flow_file_expiration: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureObjectThreshold", skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureDataSizeThreshold", skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// The number of running components in this process group.
    #[serde(rename = "runningCount", skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stopped components in the process group.
    #[serde(rename = "stoppedCount", skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of invalid components in the process group.
    #[serde(rename = "invalidCount", skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of disabled components in the process group.
    #[serde(rename = "disabledCount", skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The number of active remote ports in the process group.
    #[serde(rename = "activeRemotePortCount", skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The number of inactive remote ports in the process group.
    #[serde(rename = "inactiveRemotePortCount", skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    #[serde(rename = "upToDateCount", skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
    /// The number of locally modified versioned process groups in the process group.
    #[serde(rename = "locallyModifiedCount", skip_serializing_if = "Option::is_none")]
    pub locally_modified_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    #[serde(rename = "staleCount", skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the process group.
    #[serde(rename = "locallyModifiedAndStaleCount", skip_serializing_if = "Option::is_none")]
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    #[serde(rename = "syncFailureCount", skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of local input ports in the process group.
    #[serde(rename = "localInputPortCount", skip_serializing_if = "Option::is_none")]
    pub local_input_port_count: Option<i32>,
    /// The number of local output ports in the process group.
    #[serde(rename = "localOutputPortCount", skip_serializing_if = "Option::is_none")]
    pub local_output_port_count: Option<i32>,
    /// The number of public input ports in the process group.
    #[serde(rename = "publicInputPortCount", skip_serializing_if = "Option::is_none")]
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    #[serde(rename = "publicOutputPortCount", skip_serializing_if = "Option::is_none")]
    pub public_output_port_count: Option<i32>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Box<crate::models::FlowSnippetDto>>,
    /// The number of input ports in the process group.
    #[serde(rename = "inputPortCount", skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The number of output ports in the process group.
    #[serde(rename = "outputPortCount", skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
}

impl ProcessGroupDto {
    pub fn new() -> ProcessGroupDto {
        ProcessGroupDto {
            id: None,
            versioned_component_id: None,
            parent_group_id: None,
            position: None,
            name: None,
            comments: None,
            variables: None,
            version_control_information: None,
            parameter_context: None,
            flowfile_concurrency: None,
            flowfile_outbound_policy: None,
            default_flow_file_expiration: None,
            default_back_pressure_object_threshold: None,
            default_back_pressure_data_size_threshold: None,
            running_count: None,
            stopped_count: None,
            invalid_count: None,
            disabled_count: None,
            active_remote_port_count: None,
            inactive_remote_port_count: None,
            up_to_date_count: None,
            locally_modified_count: None,
            stale_count: None,
            locally_modified_and_stale_count: None,
            sync_failure_count: None,
            local_input_port_count: None,
            local_output_port_count: None,
            public_input_port_count: None,
            public_output_port_count: None,
            contents: None,
            input_port_count: None,
            output_port_count: None,
        }
    }
}

/// The FlowFile Concurrency for this Process Group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowfileConcurrency {
    #[serde(rename = "UNBOUNDED")]
    UNBOUNDED,
    #[serde(rename = "SINGLE_FLOWFILE_PER_NODE")]
    SINGLEFLOWFILEPERNODE,
}

impl Default for FlowfileConcurrency {
    fn default() -> FlowfileConcurrency {
        Self::UNBOUNDED
    }
}
/// The Outbound Policy that is used for determining how FlowFiles should be transferred out of the Process Group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowfileOutboundPolicy {
    #[serde(rename = "STREAM_WHEN_AVAILABLE")]
    STREAMWHENAVAILABLE,
    #[serde(rename = "BATCH_OUTPUT")]
    BATCHOUTPUT,
}

impl Default for FlowfileOutboundPolicy {
    fn default() -> FlowfileOutboundPolicy {
        Self::STREAMWHENAVAILABLE
    }
}
