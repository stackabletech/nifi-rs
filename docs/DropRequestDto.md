# DropRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id for this drop request. | [optional]
**uri** | Option<**String**> | The URI for future requests to this drop request. | [optional]
**submission_time** | Option<**String**> | The timestamp when the query was submitted. | [optional]
**last_updated** | Option<**String**> | The last time this drop request was updated. | [optional]
**percent_completed** | Option<**i32**> | The current percent complete. | [optional]
**finished** | Option<**bool**> | Whether the query has finished. | [optional]
**failure_reason** | Option<**String**> | The reason, if any, that this drop request failed. | [optional]
**current_count** | Option<**i32**> | The number of flow files currently queued. | [optional]
**current_size** | Option<**i64**> | The size of flow files currently queued in bytes. | [optional]
**current** | Option<**String**> | The count and size of flow files currently queued. | [optional]
**original_count** | Option<**i32**> | The number of flow files to be dropped as a result of this request. | [optional]
**original_size** | Option<**i64**> | The size of flow files to be dropped as a result of this request in bytes. | [optional]
**original** | Option<**String**> | The count and size of flow files to be dropped as a result of this request. | [optional]
**dropped_count** | Option<**i32**> | The number of flow files that have been dropped thus far. | [optional]
**dropped_size** | Option<**i64**> | The size of flow files that have been dropped thus far in bytes. | [optional]
**dropped** | Option<**String**> | The count and size of flow files that have been dropped thus far. | [optional]
**state** | Option<**String**> | The current state of the drop request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


