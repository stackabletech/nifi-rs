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
pub struct ProcessorDefinition {
    /// The group name of the bundle that provides the referenced type.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub _type: String,
    /// The description of the type.
    #[serde(rename = "typeDescription", skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    #[serde(rename = "buildInfo", skip_serializing_if = "Option::is_none")]
    pub build_info: Option<Box<crate::models::BuildInfo>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations", skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<crate::models::DefinedType>>,
    /// The tags associated with this type
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether or not the component has been deprecated
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason", skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors", skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<::std::collections::HashMap<String, crate::models::PropertyDescriptor>>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties", skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Any input requirements this processor has.
    #[serde(rename = "inputRequirement", skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<InputRequirement>,
    /// The supported relationships for this processor.
    #[serde(rename = "supportedRelationships", skip_serializing_if = "Option::is_none")]
    pub supported_relationships: Option<Vec<crate::models::Relationship>>,
    /// Whether or not this processor supports dynamic relationships.
    #[serde(rename = "supportsDynamicRelationships", skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_relationships: Option<bool>,
    /// Whether or not this processor should be triggered serially (i.e. no concurrent execution).
    #[serde(rename = "triggerSerially", skip_serializing_if = "Option::is_none")]
    pub trigger_serially: Option<bool>,
    /// Whether or not this processor should be triggered when incoming queues are empty.
    #[serde(rename = "triggerWhenEmpty", skip_serializing_if = "Option::is_none")]
    pub trigger_when_empty: Option<bool>,
    /// Whether or not this processor should be triggered when any destination queue has room.
    #[serde(rename = "triggerWhenAnyDestinationAvailable", skip_serializing_if = "Option::is_none")]
    pub trigger_when_any_destination_available: Option<bool>,
    /// Whether or not this processor supports batching. If a Processor uses this annotation, it allows the Framework to batch calls to session commits, as well as allowing the Framework to return the same session multiple times.
    #[serde(rename = "supportsBatching", skip_serializing_if = "Option::is_none")]
    pub supports_batching: Option<bool>,
    /// Whether or not this processor supports event driven scheduling. Indicates to the framework that the Processor is eligible to be scheduled to run based on the occurrence of an \"Event\" (e.g., when a FlowFile is enqueued in an incoming Connection), rather than being triggered periodically.
    #[serde(rename = "supportsEventDriven", skip_serializing_if = "Option::is_none")]
    pub supports_event_driven: Option<bool>,
    /// Whether or not this processor should be scheduled only on the primary node in a cluster.
    #[serde(rename = "primaryNodeOnly", skip_serializing_if = "Option::is_none")]
    pub primary_node_only: Option<bool>,
    /// Whether or not this processor is considered side-effect free. Side-effect free indicate that the processor's operations on FlowFiles can be safely repeated across process sessions.
    #[serde(rename = "sideEffectFree", skip_serializing_if = "Option::is_none")]
    pub side_effect_free: Option<bool>,
    /// The supported scheduling strategies, such as TIME_DRIVER, CRON, or EVENT_DRIVEN.
    #[serde(rename = "supportedSchedulingStrategies", skip_serializing_if = "Option::is_none")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// The default scheduling strategy for the processor.
    #[serde(rename = "defaultSchedulingStrategy", skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<String>,
    /// The default concurrent tasks for each scheduling strategy.
    #[serde(rename = "defaultConcurrentTasksBySchedulingStrategy", skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks_by_scheduling_strategy: Option<::std::collections::HashMap<String, i32>>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as \"30 sec\".
    #[serde(rename = "defaultSchedulingPeriodBySchedulingStrategy", skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_by_scheduling_strategy: Option<::std::collections::HashMap<String, String>>,
    /// The default penalty duration as a time period, such as \"30 sec\".
    #[serde(rename = "defaultPenaltyDuration", skip_serializing_if = "Option::is_none")]
    pub default_penalty_duration: Option<String>,
    /// The default yield duration as a time period, such as \"1 sec\".
    #[serde(rename = "defaultYieldDuration", skip_serializing_if = "Option::is_none")]
    pub default_yield_duration: Option<String>,
    /// The default bulletin level, such as WARN, INFO, DEBUG, etc.
    #[serde(rename = "defaultBulletinLevel", skip_serializing_if = "Option::is_none")]
    pub default_bulletin_level: Option<String>,
}

impl ProcessorDefinition {
    pub fn new(_type: String) -> ProcessorDefinition {
        ProcessorDefinition {
            group: None,
            artifact: None,
            version: None,
            _type,
            type_description: None,
            build_info: None,
            provided_api_implementations: None,
            tags: None,
            deprecated: None,
            deprecation_reason: None,
            property_descriptors: None,
            supports_dynamic_properties: None,
            input_requirement: None,
            supported_relationships: None,
            supports_dynamic_relationships: None,
            trigger_serially: None,
            trigger_when_empty: None,
            trigger_when_any_destination_available: None,
            supports_batching: None,
            supports_event_driven: None,
            primary_node_only: None,
            side_effect_free: None,
            supported_scheduling_strategies: None,
            default_scheduling_strategy: None,
            default_concurrent_tasks_by_scheduling_strategy: None,
            default_scheduling_period_by_scheduling_strategy: None,
            default_penalty_duration: None,
            default_yield_duration: None,
            default_bulletin_level: None,
        }
    }
}

/// Any input requirements this processor has.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InputRequirement {
    #[serde(rename = "INPUT_REQUIRED")]
    REQUIRED,
    #[serde(rename = "INPUT_ALLOWED")]
    ALLOWED,
    #[serde(rename = "INPUT_FORBIDDEN")]
    FORBIDDEN,
}

impl Default for InputRequirement {
    fn default() -> InputRequirement {
        Self::REQUIRED
    }
}
