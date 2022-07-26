# VersionedFlow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link** | Option<[**crate::models::JaxbLink**](JaxbLink.md)> |  | [optional]
**identifier** | Option<**String**> | An ID to uniquely identify this object. | [optional][readonly]
**name** | **String** | The name of the item. | 
**description** | Option<**String**> | A description of the item. | [optional]
**bucket_identifier** | **String** | The identifier of the bucket this items belongs to. This cannot be changed after the item is created. | 
**bucket_name** | Option<**String**> | The name of the bucket this items belongs to. | [optional][readonly]
**created_timestamp** | Option<**i64**> | The timestamp of when the item was created, as milliseconds since epoch. | [optional][readonly]
**modified_timestamp** | Option<**i64**> | The timestamp of when the item was last modified, as milliseconds since epoch. | [optional][readonly]
**_type** | **String** | The type of item. | 
**permissions** | Option<[**crate::models::Permissions**](Permissions.md)> |  | [optional]
**version_count** | Option<**i64**> | The number of versions of this flow. | [optional][readonly]
**revision** | Option<[**crate::models::RevisionInfo**](RevisionInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


