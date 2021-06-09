# V1CronJobSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**concurrency_policy** | Option<**String**> | Specifies how to treat concurrent executions of a Job. Valid values are: - \"Allow\" (default): allows CronJobs to run concurrently; - \"Forbid\": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - \"Replace\": cancels currently running job and replaces it with a new one | [optional]
**failed_jobs_history_limit** | Option<**i32**> | The number of failed finished jobs to retain. Value must be non-negative integer. Defaults to 1. | [optional]
**job_template** | [**crate::models::V1JobTemplateSpec**](v1.JobTemplateSpec.md) |  | 
**schedule** | **String** | The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron. | 
**starting_deadline_seconds** | Option<**i64**> | Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones. | [optional]
**successful_jobs_history_limit** | Option<**i32**> | The number of successful finished jobs to retain. Value must be non-negative integer. Defaults to 3. | [optional]
**suspend** | Option<**bool**> | This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


