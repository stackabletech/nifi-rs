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
pub struct SearchResultsDto {
    /// The processors that matched the search.
    #[serde(rename = "processorResults", skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The connections that matched the search.
    #[serde(rename = "connectionResults", skip_serializing_if = "Option::is_none")]
    pub connection_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The process groups that matched the search.
    #[serde(rename = "processGroupResults", skip_serializing_if = "Option::is_none")]
    pub process_group_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The input ports that matched the search.
    #[serde(rename = "inputPortResults", skip_serializing_if = "Option::is_none")]
    pub input_port_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The output ports that matched the search.
    #[serde(rename = "outputPortResults", skip_serializing_if = "Option::is_none")]
    pub output_port_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The remote process groups that matched the search.
    #[serde(rename = "remoteProcessGroupResults", skip_serializing_if = "Option::is_none")]
    pub remote_process_group_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The funnels that matched the search.
    #[serde(rename = "funnelResults", skip_serializing_if = "Option::is_none")]
    pub funnel_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The labels that matched the search.
    #[serde(rename = "labelResults", skip_serializing_if = "Option::is_none")]
    pub label_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The controller service nodes that matched the search
    #[serde(rename = "controllerServiceNodeResults", skip_serializing_if = "Option::is_none")]
    pub controller_service_node_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The parameter contexts that matched the search.
    #[serde(rename = "parameterContextResults", skip_serializing_if = "Option::is_none")]
    pub parameter_context_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
    /// The parameters that matched the search.
    #[serde(rename = "parameterResults", skip_serializing_if = "Option::is_none")]
    pub parameter_results: Option<Vec<crate::models::ComponentSearchResultDto>>,
}

impl SearchResultsDto {
    pub fn new() -> SearchResultsDto {
        SearchResultsDto {
            processor_results: None,
            connection_results: None,
            process_group_results: None,
            input_port_results: None,
            output_port_results: None,
            remote_process_group_results: None,
            funnel_results: None,
            label_results: None,
            controller_service_node_results: None,
            parameter_context_results: None,
            parameter_results: None,
        }
    }
}


