# ProvenanceRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**search_terms** | Option<[**::std::collections::HashMap<String, crate::models::ProvenanceSearchValueDto>**](ProvenanceSearchValueDTO.md)> | The search terms used to perform the search. | [optional]
**cluster_node_id** | Option<**String**> | The id of the node in the cluster where this provenance originated. | [optional]
**start_date** | Option<**String**> | The earliest event time to include in the query. | [optional]
**end_date** | Option<**String**> | The latest event time to include in the query. | [optional]
**minimum_file_size** | Option<**String**> | The minimum file size to include in the query. | [optional]
**maximum_file_size** | Option<**String**> | The maximum file size to include in the query. | [optional]
**max_results** | Option<**i32**> | The maximum number of results to include. | [optional]
**summarize** | Option<**bool**> | Whether or not to summarize provenance events returned. This property is false by default. | [optional]
**incremental_results** | Option<**bool**> | Whether or not incremental results are returned. If false, provenance events are only returned once the query completes. This property is true by default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


