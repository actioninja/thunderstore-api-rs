# \ApiApi

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_experimental_community_category_list**](ApiApi.md#api_experimental_community_category_list) | **GET** /api/experimental/community/{community}/category/ |
[**api_experimental_community_list**](ApiApi.md#api_experimental_community_list) | **GET** /api/experimental/community/ |
[**api_experimental_current_user_list**](ApiApi.md#api_experimental_current_user_list) | **GET** /api/experimental/current-user/ |
[**api_experimental_package_list**](ApiApi.md#api_experimental_package_list) | **GET** /api/experimental/package/ |
[**api_experimental_submission_upload_create**](ApiApi.md#api_experimental_submission_upload_create) | **POST** /api/experimental/submission/upload/ |
[**api_experimental_submission_upload_list**](ApiApi.md#api_experimental_submission_upload_list) | **GET** /api/experimental/submission/upload/ |
[**api_v1_bot_deprecate_mod_create**](ApiApi.md#api_v1_bot_deprecate_mod_create) | **POST** /api/v1/bot/deprecate-mod/ | Deprecates a mod by it's package name
[**api_v1_current_user_info_list**](ApiApi.md#api_v1_current_user_info_list) | **GET** /api/v1/current-user/info/ |
[**api_v1_package_list**](ApiApi.md#api_v1_package_list) | **GET** /api/v1/package/ |
[**api_v1_package_rate**](ApiApi.md#api_v1_package_rate) | **POST** /api/v1/package/{uuid4}/rate/ |
[**api_v1_package_read**](ApiApi.md#api_v1_package_read) | **GET** /api/v1/package/{uuid4}/ |
[**experimental_package_read**](ApiApi.md#experimental_package_read) | **GET** /api/experimental/package/{namespace}/{name}/ |
[**experimental_package_version_read**](ApiApi.md#experimental_package_version_read) | **GET** /api/experimental/package/{namespace}/{name}/{version}/ |
[**experimental_period_auth_period_complete**](ApiApi.md#experimental_period_auth_period_complete) | **POST** /api/experimental/auth/complete/{provider}/ |
[**experimental_period_auth_period_validate**](ApiApi.md#experimental_period_auth_period_validate) | **GET** /api/experimental/auth/validate/ |
[**experimental_period_community_period_current**](ApiApi.md#experimental_period_community_period_current) | **GET** /api/experimental/current-community/ |
[**experimental_period_frontend_period_community_period_package**](ApiApi.md#experimental_period_frontend_period_community_period_package) | **GET** /api/experimental/frontend/c/{community_identifier}/p/{package_namespace}/{package_name}/ |
[**experimental_period_frontend_period_community_period_packages**](ApiApi.md#experimental_period_frontend_period_community_period_packages) | **GET** /api/experimental/frontend/c/{community_identifier}/packages/ |
[**experimental_period_frontend_period_frontpage**](ApiApi.md#experimental_period_frontend_period_frontpage) | **GET** /api/experimental/frontend/frontpage/ |
[**experimental_period_frontend_period_render_markdown**](ApiApi.md#experimental_period_frontend_period_render_markdown) | **POST** /api/experimental/frontend/render-markdown/ |
[**experimental_period_package_period_submit**](ApiApi.md#experimental_period_package_period_submit) | **POST** /api/experimental/submission/submit/ |
[**experimental_period_submission_period_validate_period_icon**](ApiApi.md#experimental_period_submission_period_validate_period_icon) | **POST** /api/experimental/submission/validate/icon/ |
[**experimental_period_submission_period_validate_period_manifest_v1**](ApiApi.md#experimental_period_submission_period_validate_period_manifest_v1) | **POST** /api/experimental/submission/validate/manifest-v1/ |
[**experimental_period_submission_period_validate_period_readme**](ApiApi.md#experimental_period_submission_period_validate_period_readme) | **POST** /api/experimental/submission/validate/readme/ |
[**experimental_period_usermedia_period_abort_upload**](ApiApi.md#experimental_period_usermedia_period_abort_upload) | **POST** /api/experimental/usermedia/{uuid}/abort-upload/ |
[**experimental_period_usermedia_period_finish_upload**](ApiApi.md#experimental_period_usermedia_period_finish_upload) | **POST** /api/experimental/usermedia/{uuid}/finish-upload/ |
[**experimental_period_usermedia_period_initiate_upload**](ApiApi.md#experimental_period_usermedia_period_initiate_upload) | **POST** /api/experimental/usermedia/initiate-upload/ |

## api_experimental_community_category_list

> crate::models::ApiExperimentalCommunityCategoryList200Response
> api_experimental_community_category_list(community, cursor)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community** | **String** |  | [required] |
**cursor** | Option<**String**> | The pagination cursor value. |  |

### Return type

[**crate::models::
ApiExperimentalCommunityCategoryList200Response**](api_experimental_community_category_list_200_response.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_experimental_community_list

> crate::models::ApiExperimentalCommunityList200Response api_experimental_community_list(cursor)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor value. |  |

### Return type

[**crate::models::
ApiExperimentalCommunityList200Response**](api_experimental_community_list_200_response.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_experimental_current_user_list

> api_experimental_current_user_list()


Gets information about the current user, such as rated packages and permissions

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_experimental_package_list

> crate::models::ApiExperimentalPackageList200Response api_experimental_package_list(cursor)


Lists all available packages

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor value. |  |

### Return type

[**crate::models::
ApiExperimentalPackageList200Response**](api_experimental_package_list_200_response.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_experimental_submission_upload_create

> api_experimental_submission_upload_create()


Uploads a package. Requires multipart/form-data.

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_experimental_submission_upload_list

> api_experimental_submission_upload_list()


Uploads a package. Requires multipart/form-data.

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_v1_bot_deprecate_mod_create

> api_v1_bot_deprecate_mod_create()
> Deprecates a mod by it's package name

* Requires JWT authentication. * Only users with special permissions may use this action

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_v1_current_user_info_list

> api_v1_current_user_info_list()


Gets information about the current user, such as rated packages and permissions

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_v1_package_list

> Vec<crate::models::PackageListing> api_v1_package_list()

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PackageListing>**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## api_v1_package_rate

> crate::models::PackageListing api_v1_package_rate(uuid4, data)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

## api_v1_package_read

> crate::models::PackageListing api_v1_package_read(uuid4)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid4** | **String** |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_package_read

> crate::models::PackageExperimental experimental_package_read(name, namespace)


Get a single package

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**crate::models::PackageExperimental**](PackageExperimental.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_package_version_read

> crate::models::PackageVersionExperimental experimental_package_version_read(name, namespace,
> version)


Get a single package version

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**crate::models::PackageVersionExperimental**](PackageVersionExperimental.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_auth_period_complete

> crate::models::ResponseBody experimental_period_auth_period_complete(provider, data)


Complete OAuth login process initiated by a client.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** |  | [required] |
**data** | [**RequestBody**](RequestBody.md) |  | [required] |

### Return type

[**crate::models::ResponseBody**](ResponseBody.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_auth_period_validate

> experimental_period_auth_period_validate()


Check that valid session key is provided in Authorization header.

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_community_period_current

> crate::models::Community experimental_period_community_period_current()


Fetch the Community of the queried domain

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Community**](Community.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_frontend_period_community_period_package

> crate::models::PackageDetailViewContent
> experimental_period_frontend_period_community_period_package(community_identifier, package_name,
> package_namespace)


Return details about a single Package.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_namespace** | **String** |  | [required] |

### Return type

[**crate::models::PackageDetailViewContent**](PackageDetailViewContent.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_frontend_period_community_period_packages

> crate::models::CommunityPackageList experimental_period_frontend_period_community_period_packages(
> community_identifier)


Return paginated list of community's packages.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

[**crate::models::CommunityPackageList**](CommunityPackageList.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_frontend_period_frontpage

> crate::models::FrontPageContent experimental_period_frontend_period_frontpage()


Return information required to render the site's front page.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FrontPageContent**](FrontPageContent.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_frontend_period_render_markdown

> crate::models::RenderMarkdownResponse experimental_period_frontend_period_render_markdown(data)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**RenderMarkdownParams**](RenderMarkdownParams.md) |  | [required] |

### Return type

[**crate::models::RenderMarkdownResponse**](RenderMarkdownResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_package_period_submit

> crate::models::PackageSubmissionResult experimental_period_package_period_submit(data)


Submits a pre-uploaded package by upload uuid.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**PackageSubmissionMetadata**](PackageSubmissionMetadata.md) |  | [required] |

### Return type

[**crate::models::PackageSubmissionResult**](PackageSubmissionResult.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_submission_period_validate_period_icon

> crate::models::ValidatorResponse experimental_period_submission_period_validate_period_icon(data)


Validates a package icon.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**IconValidatorParams**](IconValidatorParams.md) |  | [required] |

### Return type

[**crate::models::ValidatorResponse**](ValidatorResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_submission_period_validate_period_manifest_v1

> crate::models::ValidatorResponse
> experimental_period_submission_period_validate_period_manifest_v1(data)


Validates a package manifest.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ManifestV1ValidatorParams**](ManifestV1ValidatorParams.md) |  | [required] |

### Return type

[**crate::models::ValidatorResponse**](ValidatorResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_submission_period_validate_period_readme

> crate::models::ValidatorResponse experimental_period_submission_period_validate_period_readme(
> data)


Validates a package readme.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ReadmeValidatorParams**](ReadmeValidatorParams.md) |  | [required] |

### Return type

[**crate::models::ValidatorResponse**](ValidatorResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_usermedia_period_abort_upload

> crate::models::UserMedia experimental_period_usermedia_period_abort_upload(uuid)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this user media. | [required] |

### Return type

[**crate::models::UserMedia**](UserMedia.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_usermedia_period_finish_upload

> crate::models::UserMedia experimental_period_usermedia_period_finish_upload(uuid, data)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this user media. | [required] |
**data** | [**UserMediaFinishUploadParams**](UserMediaFinishUploadParams.md) |  | [required] |

### Return type

[**crate::models::UserMedia**](UserMedia.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## experimental_period_usermedia_period_initiate_upload

> crate::models::UserMediaInitiateUploadResponse
> experimental_period_usermedia_period_initiate_upload(data)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**UserMediaInitiateUploadParams**](UserMediaInitiateUploadParams.md) |  | [required] |

### Return type

[**crate::models::UserMediaInitiateUploadResponse**](UserMediaInitiateUploadResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

