# Bucket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link** | Option<[**crate::models::JaxbLink**](JaxbLink.md)> |  | [optional]
**identifier** | Option<**String**> | An ID to uniquely identify this object. | [optional][readonly]
**name** | **String** | The name of the bucket. | 
**created_timestamp** | Option<**i64**> | The timestamp of when the bucket was first created. This is set by the server at creation time. | [optional][readonly]
**description** | Option<**String**> | A description of the bucket. | [optional]
**allow_bundle_redeploy** | Option<**bool**> | Indicates if this bucket allows the same version of an extension bundle to be redeployed and thus overwrite the existing artifact. By default this is false. | [optional]
**allow_public_read** | Option<**bool**> | Indicates if this bucket allows read access to unauthenticated anonymous users | [optional]
**permissions** | Option<[**crate::models::Permissions**](Permissions.md)> |  | [optional]
**revision** | Option<[**crate::models::RevisionInfo**](RevisionInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


