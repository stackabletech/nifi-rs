# RepositoryUsageDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the repository | [optional]
**file_store_hash** | Option<**String**> | A SHA-256 hash of the File Store name/path that is used to store the repository's data. This information is exposed as a hash in order to avoid exposing potentially sensitive information that is not generally relevant. What is typically relevant is whether or not multiple repositories on the same node are using the same File Store, as this indicates that the repositories are competing for the resources of the backing disk/storage mechanism. | [optional]
**free_space** | Option<**String**> | Amount of free space. | [optional]
**total_space** | Option<**String**> | Amount of total space. | [optional]
**free_space_bytes** | Option<**i64**> | The number of bytes of free space. | [optional]
**total_space_bytes** | Option<**i64**> | The number of bytes of total space. | [optional]
**utilization** | Option<**String**> | Utilization of this storage location. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


