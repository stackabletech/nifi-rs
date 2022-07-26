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
pub struct VersionedProcessor {
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
    /// The type of the extension component
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "bundle", skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Box<crate::models::Bundle>>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
    /// The property descriptors for the component.
    #[serde(rename = "propertyDescriptors", skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<::std::collections::HashMap<String, crate::models::VersionedPropertyDescriptor>>,
    /// Stylistic data for rendering in a UI
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<::std::collections::HashMap<String, String>>,
    /// The annotation data for the processor used to relay configuration between a custom UI and the procesosr.
    #[serde(rename = "annotationData", skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    #[serde(rename = "schedulingPeriod", skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates whether the processor should be scheduled to run in event or timer driven mode.
    #[serde(rename = "schedulingStrategy", skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Indicates the node where the process will execute.
    #[serde(rename = "executionNode", skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<String>,
    /// The amout of time that is used when the process penalizes a flowfile.
    #[serde(rename = "penaltyDuration", skip_serializing_if = "Option::is_none")]
    pub penalty_duration: Option<String>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    #[serde(rename = "yieldDuration", skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
    /// The level at which the processor will report bulletins.
    #[serde(rename = "bulletinLevel", skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    /// The run duration for the processor in milliseconds.
    #[serde(rename = "runDurationMillis", skip_serializing_if = "Option::is_none")]
    pub run_duration_millis: Option<i64>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    #[serde(rename = "concurrentlySchedulableTaskCount", skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    #[serde(rename = "autoTerminatedRelationships", skip_serializing_if = "Option::is_none")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// The scheduled state of the component
    #[serde(rename = "scheduledState", skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<ScheduledState>,
    /// Overall number of retries.
    #[serde(rename = "retryCount", skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    /// All the relationships should be retried.
    #[serde(rename = "retriedRelationships", skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries.
    #[serde(rename = "backoffMechanism", skip_serializing_if = "Option::is_none")]
    pub backoff_mechanism: Option<BackoffMechanism>,
    /// Maximum amount of time to be waited during a retry period.
    #[serde(rename = "maxBackoffPeriod", skip_serializing_if = "Option::is_none")]
    pub max_backoff_period: Option<String>,
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<ComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier", skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
}

impl VersionedProcessor {
    pub fn new() -> VersionedProcessor {
        VersionedProcessor {
            identifier: None,
            instance_identifier: None,
            name: None,
            comments: None,
            position: None,
            _type: None,
            bundle: None,
            properties: None,
            property_descriptors: None,
            style: None,
            annotation_data: None,
            scheduling_period: None,
            scheduling_strategy: None,
            execution_node: None,
            penalty_duration: None,
            yield_duration: None,
            bulletin_level: None,
            run_duration_millis: None,
            concurrently_schedulable_task_count: None,
            auto_terminated_relationships: None,
            scheduled_state: None,
            retry_count: None,
            retried_relationships: None,
            backoff_mechanism: None,
            max_backoff_period: None,
            component_type: None,
            group_identifier: None,
        }
    }
}

/// The scheduled state of the component
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScheduledState {
    #[serde(rename = "ENABLED")]
    ENABLED,
    #[serde(rename = "DISABLED")]
    DISABLED,
    #[serde(rename = "RUNNING")]
    RUNNING,
}

impl Default for ScheduledState {
    fn default() -> ScheduledState {
        Self::ENABLED
    }
}
/// Determines whether the FlowFile should be penalized or the processor should be yielded between retries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackoffMechanism {
    #[serde(rename = "PENALIZE_FLOWFILE")]
    PENALIZEFLOWFILE,
    #[serde(rename = "YIELD_PROCESSOR")]
    YIELDPROCESSOR,
}

impl Default for BackoffMechanism {
    fn default() -> BackoffMechanism {
        Self::PENALIZEFLOWFILE
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
