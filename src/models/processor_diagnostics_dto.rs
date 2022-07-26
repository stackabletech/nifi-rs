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
pub struct ProcessorDiagnosticsDto {
    #[serde(rename = "processor", skip_serializing_if = "Option::is_none")]
    pub processor: Option<Box<crate::models::ProcessorDto>>,
    #[serde(rename = "processorStatus", skip_serializing_if = "Option::is_none")]
    pub processor_status: Option<Box<crate::models::ProcessorStatusDto>>,
    /// Diagnostic Information about all Controller Services that the Processor is referencing
    #[serde(rename = "referencedControllerServices", skip_serializing_if = "Option::is_none")]
    pub referenced_controller_services: Option<Vec<crate::models::ControllerServiceDiagnosticsDto>>,
    /// Diagnostic Information about all incoming Connections
    #[serde(rename = "incomingConnections", skip_serializing_if = "Option::is_none")]
    pub incoming_connections: Option<Vec<crate::models::ConnectionDiagnosticsDto>>,
    /// Diagnostic Information about all outgoing Connections
    #[serde(rename = "outgoingConnections", skip_serializing_if = "Option::is_none")]
    pub outgoing_connections: Option<Vec<crate::models::ConnectionDiagnosticsDto>>,
    #[serde(rename = "jvmDiagnostics", skip_serializing_if = "Option::is_none")]
    pub jvm_diagnostics: Option<Box<crate::models::JvmDiagnosticsDto>>,
    /// Thread Dumps that were taken of the threads that are active in the Processor
    #[serde(rename = "threadDumps", skip_serializing_if = "Option::is_none")]
    pub thread_dumps: Option<Vec<crate::models::ThreadDumpDto>>,
    #[serde(rename = "classLoaderDiagnostics", skip_serializing_if = "Option::is_none")]
    pub class_loader_diagnostics: Option<Box<crate::models::ClassLoaderDiagnosticsDto>>,
}

impl ProcessorDiagnosticsDto {
    pub fn new() -> ProcessorDiagnosticsDto {
        ProcessorDiagnosticsDto {
            processor: None,
            processor_status: None,
            referenced_controller_services: None,
            incoming_connections: None,
            outgoing_connections: None,
            jvm_diagnostics: None,
            thread_dumps: None,
            class_loader_diagnostics: None,
        }
    }
}

