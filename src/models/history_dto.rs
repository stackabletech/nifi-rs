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
pub struct HistoryDto {
    /// The number of number of actions that matched the search criteria..
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The timestamp when the report was generated.
    #[serde(rename = "lastRefreshed", skip_serializing_if = "Option::is_none")]
    pub last_refreshed: Option<String>,
    /// The actions.
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::models::ActionEntity>>,
}

impl HistoryDto {
    pub fn new() -> HistoryDto {
        HistoryDto {
            total: None,
            last_refreshed: None,
            actions: None,
        }
    }
}


