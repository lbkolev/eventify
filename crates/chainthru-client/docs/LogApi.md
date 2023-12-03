# \LogApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logs_count**](LogApi.md#get_logs_count) | **GET** /api/v1/logs/count | Get the Count of Logs



## get_logs_count

> get_logs_count()
Get the Count of Logs

Get the Count of Logs  This endpoint returns the total count of logs present in the database. The response is a JSON object containing the count.  # Responses  * `200 OK`: Successfully retrieved the count of logs. The response body will be a JSON object with the structure `{ \"count\": i64 }`, where `i64` is the total number of logs. * `500 Internal Server Error`: Indicates that an error occurred on the server while processing the request. The response body will contain a JSON object with an error message.  # Example  ```json { \"count\": 456 } ```

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

