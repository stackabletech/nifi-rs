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
pub struct ProcessorRunStatusDetailsEntity {
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Box<crate::models::RevisionDto>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::PermissionsDto>>,
    #[serde(rename = "runStatusDetails", skip_serializing_if = "Option::is_none")]
    pub run_status_details: Option<Box<crate::models::ProcessorRunStatusDetailsDto>>,
}

impl ProcessorRunStatusDetailsEntity {
    pub fn new() -> ProcessorRunStatusDetailsEntity {
        ProcessorRunStatusDetailsEntity {
            revision: None,
            permissions: None,
            run_status_details: None,
        }
    }
}


