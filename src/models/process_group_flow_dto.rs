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
pub struct ProcessGroupFlowDto {
    /// The id of the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI for futures requests to the component.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId", skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "parameterContext", skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<Box<crate::models::ParameterContextReferenceEntity>>,
    #[serde(rename = "breadcrumb", skip_serializing_if = "Option::is_none")]
    pub breadcrumb: Option<Box<crate::models::FlowBreadcrumbEntity>>,
    #[serde(rename = "flow", skip_serializing_if = "Option::is_none")]
    pub flow: Option<Box<crate::models::FlowDto>>,
    /// The time the flow for the process group was last refreshed.
    #[serde(rename = "lastRefreshed", skip_serializing_if = "Option::is_none")]
    pub last_refreshed: Option<String>,
}

impl ProcessGroupFlowDto {
    pub fn new() -> ProcessGroupFlowDto {
        ProcessGroupFlowDto {
            id: None,
            uri: None,
            parent_group_id: None,
            parameter_context: None,
            breadcrumb: None,
            flow: None,
            last_refreshed: None,
        }
    }
}


