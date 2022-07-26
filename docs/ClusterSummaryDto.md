# ClusterSummaryDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connected_nodes** | Option<**String**> | When clustered, reports the number of nodes connected vs the number of nodes in the cluster. | [optional]
**connected_node_count** | Option<**i32**> | The number of nodes that are currently connected to the cluster | [optional]
**total_node_count** | Option<**i32**> | The number of nodes in the cluster, regardless of whether or not they are connected | [optional]
**clustered** | Option<**bool**> | Whether this NiFi instance is clustered. | [optional]
**connected_to_cluster** | Option<**bool**> | Whether this NiFi instance is connected to a cluster. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


