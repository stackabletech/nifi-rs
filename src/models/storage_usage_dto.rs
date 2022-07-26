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
pub struct StorageUsageDto {
    /// The identifier of this storage location. The identifier will correspond to the identifier keyed in the storage configuration.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Amount of free space.
    #[serde(rename = "freeSpace", skip_serializing_if = "Option::is_none")]
    pub free_space: Option<String>,
    /// Amount of total space.
    #[serde(rename = "totalSpace", skip_serializing_if = "Option::is_none")]
    pub total_space: Option<String>,
    /// Amount of used space.
    #[serde(rename = "usedSpace", skip_serializing_if = "Option::is_none")]
    pub used_space: Option<String>,
    /// The number of bytes of free space.
    #[serde(rename = "freeSpaceBytes", skip_serializing_if = "Option::is_none")]
    pub free_space_bytes: Option<i64>,
    /// The number of bytes of total space.
    #[serde(rename = "totalSpaceBytes", skip_serializing_if = "Option::is_none")]
    pub total_space_bytes: Option<i64>,
    /// The number of bytes of used space.
    #[serde(rename = "usedSpaceBytes", skip_serializing_if = "Option::is_none")]
    pub used_space_bytes: Option<i64>,
    /// Utilization of this storage location.
    #[serde(rename = "utilization", skip_serializing_if = "Option::is_none")]
    pub utilization: Option<String>,
}

impl StorageUsageDto {
    pub fn new() -> StorageUsageDto {
        StorageUsageDto {
            identifier: None,
            free_space: None,
            total_space: None,
            used_space: None,
            free_space_bytes: None,
            total_space_bytes: None,
            used_space_bytes: None,
            utilization: None,
        }
    }
}


