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
pub struct ComponentManifest {
    /// Public interfaces defined in this bundle
    #[serde(rename = "apis", skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<crate::models::DefinedType>>,
    /// Controller Services provided in this bundle
    #[serde(rename = "controllerServices", skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<crate::models::ControllerServiceDefinition>>,
    /// Processors provided in this bundle
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::models::ProcessorDefinition>>,
    /// Reporting Tasks provided in this bundle
    #[serde(rename = "reportingTasks", skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<crate::models::ReportingTaskDefinition>>,
}

impl ComponentManifest {
    pub fn new() -> ComponentManifest {
        ComponentManifest {
            apis: None,
            controller_services: None,
            processors: None,
            reporting_tasks: None,
        }
    }
}


