////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

/*
 * Thunderstore API
 *
 * Schema is automatically generated and not completely accurate.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserMediaInitiateUploadParams {
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "file_size_bytes")]
    pub file_size_bytes: u32,
}

impl UserMediaInitiateUploadParams {
    pub fn new(filename: String, file_size_bytes: u32) -> UserMediaInitiateUploadParams {
        UserMediaInitiateUploadParams {
            filename,
            file_size_bytes,
        }
    }
}
