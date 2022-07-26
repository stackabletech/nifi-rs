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
pub struct QueueSizeDto {
    /// The size of objects in a queue.
    #[serde(rename = "byteCount", skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
    /// The count of objects in a queue.
    #[serde(rename = "objectCount", skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i32>,
}

impl QueueSizeDto {
    pub fn new() -> QueueSizeDto {
        QueueSizeDto {
            byte_count: None,
            object_count: None,
        }
    }
}


