# ProcessorDiagnosticsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**processor** | Option<[**crate::models::ProcessorDto**](ProcessorDTO.md)> |  | [optional]
**processor_status** | Option<[**crate::models::ProcessorStatusDto**](ProcessorStatusDTO.md)> |  | [optional]
**referenced_controller_services** | Option<[**Vec<crate::models::ControllerServiceDiagnosticsDto>**](ControllerServiceDiagnosticsDTO.md)> | Diagnostic Information about all Controller Services that the Processor is referencing | [optional]
**incoming_connections** | Option<[**Vec<crate::models::ConnectionDiagnosticsDto>**](ConnectionDiagnosticsDTO.md)> | Diagnostic Information about all incoming Connections | [optional]
**outgoing_connections** | Option<[**Vec<crate::models::ConnectionDiagnosticsDto>**](ConnectionDiagnosticsDTO.md)> | Diagnostic Information about all outgoing Connections | [optional]
**jvm_diagnostics** | Option<[**crate::models::JvmDiagnosticsDto**](JVMDiagnosticsDTO.md)> |  | [optional]
**thread_dumps** | Option<[**Vec<crate::models::ThreadDumpDto>**](ThreadDumpDTO.md)> | Thread Dumps that were taken of the threads that are active in the Processor | [optional]
**class_loader_diagnostics** | Option<[**crate::models::ClassLoaderDiagnosticsDto**](ClassLoaderDiagnosticsDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


