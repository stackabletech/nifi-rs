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
pub struct GarbageCollectionDto {
    /// The name of the garbage collector.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of times garbage collection has run.
    #[serde(rename = "collectionCount", skip_serializing_if = "Option::is_none")]
    pub collection_count: Option<i64>,
    /// The total amount of time spent garbage collecting.
    #[serde(rename = "collectionTime", skip_serializing_if = "Option::is_none")]
    pub collection_time: Option<String>,
    /// The total number of milliseconds spent garbage collecting.
    #[serde(rename = "collectionMillis", skip_serializing_if = "Option::is_none")]
    pub collection_millis: Option<i64>,
}

impl GarbageCollectionDto {
    pub fn new() -> GarbageCollectionDto {
        GarbageCollectionDto {
            name: None,
            collection_count: None,
            collection_time: None,
            collection_millis: None,
        }
    }
}


