# \CApi

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**c_api_v1_bot_deprecate_mod_create**](CApi.md#c_api_v1_bot_deprecate_mod_create) | **POST** /c/{community_identifier}/api/v1/bot/deprecate-mod/ | Deprecates a mod by it's package name
[**c_api_v1_current_user_info_list**](CApi.md#c_api_v1_current_user_info_list) | **GET** /c/{community_identifier}/api/v1/current-user/info/ | 
[**c_api_v1_package_list**](CApi.md#c_api_v1_package_list) | **GET** /c/{community_identifier}/api/v1/package/ | 
[**c_api_v1_package_rate**](CApi.md#c_api_v1_package_rate) | **POST** /c/{community_identifier}/api/v1/package/{uuid4}/rate/ | 
[**c_api_v1_package_read**](CApi.md#c_api_v1_package_read) | **GET** /c/{community_identifier}/api/v1/package/{uuid4}/ | 



## c_api_v1_bot_deprecate_mod_create

> c_api_v1_bot_deprecate_mod_create(community_identifier)
Deprecates a mod by it's package name

* Requires JWT authentication. * Only users with special permissions may use this action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_current_user_info_list

> c_api_v1_current_user_info_list(community_identifier)


Gets information about the current user, such as rated packages and permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_package_list

> Vec<crate::models::PackageListing> c_api_v1_package_list(community_identifier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PackageListing>**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_package_rate

> crate::models::PackageListing c_api_v1_package_rate(community_identifier, uuid4, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**uuid4** | **String** |  | [required] |
**data** | [**PackageListing**](PackageListing.md) |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_package_read

> crate::models::PackageListing c_api_v1_package_read(community_identifier, uuid4)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**uuid4** | **String** |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

