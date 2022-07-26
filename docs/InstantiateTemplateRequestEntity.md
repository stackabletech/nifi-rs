# InstantiateTemplateRequestEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**origin_x** | Option<**f64**> | The x coordinate of the origin of the bounding box where the new components will be placed. | [optional]
**origin_y** | Option<**f64**> | The y coordinate of the origin of the bounding box where the new components will be placed. | [optional]
**template_id** | Option<**String**> | The identifier of the template. | [optional]
**encoding_version** | Option<**String**> | The encoding version of the flow snippet. If not specified, this is automatically populated by the node receiving the user request. If the snippet is specified, the version will be the latest. If the snippet is not specified, the version will come from the underlying template. These details need to be replicated throughout the cluster to ensure consistency. | [optional]
**snippet** | Option<[**crate::models::FlowSnippetDto**](FlowSnippetDTO.md)> |  | [optional]
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


