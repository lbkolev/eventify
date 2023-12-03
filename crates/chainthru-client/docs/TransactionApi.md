# \TransactionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_transactions_count**](TransactionApi.md#get_transactions_count) | **GET** /api/v1/transactions/count | Get the Count of Transactions



## get_transactions_count

> get_transactions_count()
Get the Count of Transactions

Get the Count of Transactions  This endpoint returns the total count of transactions present in the database. The response is a JSON object containing the count.  # Responses  * `200 OK`: Successfully retrieved the count of transactions. The response body will be a JSON object with the structure `{ \"count\": i64 }`, where `i64` is the total number of transactions. * `500 Internal Server Error`: Indicates that an error occurred on the server while processing the request. The response body will contain a JSON object with an error message.  # Example  ```json { \"count\": 123 } ```

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

