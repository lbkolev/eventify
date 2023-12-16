# \BlockApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_blocks_count**](BlockApi.md#get_blocks_count) | **GET** /api/v1/blocks/count | Get the Count of Blocks



## get_blocks_count

> get_blocks_count()
Get the Count of Blocks

Get the Count of Blocks  This endpoint returns the total count of blocks present in the database. The response is a JSON object containing the count.  # Responses  * `200 OK`: Successfully retrieved the count of blocks. The response body will be a JSON object with the structure `{ \"count\": i64 }`, where `i64` is the total number of blocks. * `500 Internal Server Error`: Indicates that an error occurred on the server while processing the request. The response body will contain a JSON object with an error message.  # Example  ```json { \"count\": 42 } ```

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

