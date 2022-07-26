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
pub struct FlowSnippetDto {
    /// The process groups in this flow snippet.
    #[serde(rename = "processGroups", skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<crate::models::ProcessGroupDto>>,
    /// The remote process groups in this flow snippet.
    #[serde(rename = "remoteProcessGroups", skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<crate::models::RemoteProcessGroupDto>>,
    /// The processors in this flow snippet.
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::models::ProcessorDto>>,
    /// The input ports in this flow snippet.
    #[serde(rename = "inputPorts", skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<crate::models::PortDto>>,
    /// The output ports in this flow snippet.
    #[serde(rename = "outputPorts", skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<crate::models::PortDto>>,
    /// The connections in this flow snippet.
    #[serde(rename = "connections", skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<crate::models::ConnectionDto>>,
    /// The labels in this flow snippet.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::LabelDto>>,
    /// The funnels in this flow snippet.
    #[serde(rename = "funnels", skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<crate::models::FunnelDto>>,
    /// The controller services in this flow snippet.
    #[serde(rename = "controllerServices", skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<crate::models::ControllerServiceDto>>,
}

impl FlowSnippetDto {
    pub fn new() -> FlowSnippetDto {
        FlowSnippetDto {
            process_groups: None,
            remote_process_groups: None,
            processors: None,
            input_ports: None,
            output_ports: None,
            connections: None,
            labels: None,
            funnels: None,
            controller_services: None,
        }
    }
}


