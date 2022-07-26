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
pub struct LineageDto {
    /// The id of this lineage query.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI for this lineage query for later retrieval and deletion.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// When the lineage query was submitted.
    #[serde(rename = "submissionTime", skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// When the lineage query will expire.
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// The percent complete for the lineage query.
    #[serde(rename = "percentCompleted", skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// Whether the lineage query has finished.
    #[serde(rename = "finished", skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<crate::models::LineageRequestDto>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Box<crate::models::LineageResultsDto>>,
}

impl LineageDto {
    pub fn new() -> LineageDto {
        LineageDto {
            id: None,
            uri: None,
            submission_time: None,
            expiration: None,
            percent_completed: None,
            finished: None,
            request: None,
            results: None,
        }
    }
}

