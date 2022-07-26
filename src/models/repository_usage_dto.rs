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
pub struct RepositoryUsageDto {
    /// The name of the repository
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A SHA-256 hash of the File Store name/path that is used to store the repository's data. This information is exposed as a hash in order to avoid exposing potentially sensitive information that is not generally relevant. What is typically relevant is whether or not multiple repositories on the same node are using the same File Store, as this indicates that the repositories are competing for the resources of the backing disk/storage mechanism.
    #[serde(rename = "fileStoreHash", skip_serializing_if = "Option::is_none")]
    pub file_store_hash: Option<String>,
    /// Amount of free space.
    #[serde(rename = "freeSpace", skip_serializing_if = "Option::is_none")]
    pub free_space: Option<String>,
    /// Amount of total space.
    #[serde(rename = "totalSpace", skip_serializing_if = "Option::is_none")]
    pub total_space: Option<String>,
    /// The number of bytes of free space.
    #[serde(rename = "freeSpaceBytes", skip_serializing_if = "Option::is_none")]
    pub free_space_bytes: Option<i64>,
    /// The number of bytes of total space.
    #[serde(rename = "totalSpaceBytes", skip_serializing_if = "Option::is_none")]
    pub total_space_bytes: Option<i64>,
    /// Utilization of this storage location.
    #[serde(rename = "utilization", skip_serializing_if = "Option::is_none")]
    pub utilization: Option<String>,
}

impl RepositoryUsageDto {
    pub fn new() -> RepositoryUsageDto {
        RepositoryUsageDto {
            name: None,
            file_store_hash: None,
            free_space: None,
            total_space: None,
            free_space_bytes: None,
            total_space_bytes: None,
            utilization: None,
        }
    }
}

