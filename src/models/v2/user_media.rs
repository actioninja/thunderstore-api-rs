////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UserMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    pub filename: String,
    pub size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl UserMedia {
    #[must_use]
    pub fn new(filename: String, size: u32) -> UserMedia {
        UserMedia {
            uuid: None,
            filename,
            size,
            datetime_created: None,
            expiry: None,
            status: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "upload_created")]
    UploadCreated,
    #[serde(rename = "upload_error")]
    UploadError,
    #[serde(rename = "upload_complete")]
    UploadComplete,
    #[serde(rename = "upload_aborted")]
    UploadAborted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Initial
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InitiateUploadParams {
    pub filename: String,
    pub file_size_bytes: u32,
}

impl InitiateUploadParams {
    #[must_use]
    pub fn new(filename: String, file_size_bytes: u32) -> InitiateUploadParams {
        InitiateUploadParams {
            filename,
            file_size_bytes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InitiateUploadResponse {
    pub user_media: UserMedia,
    pub upload_urls: Vec<UploadPartUrl>,
}

impl InitiateUploadResponse {
    #[must_use]
    pub fn new(user_media: UserMedia, upload_urls: Vec<UploadPartUrl>) -> InitiateUploadResponse {
        InitiateUploadResponse {
            user_media,
            upload_urls,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FinishUploadParams {
    pub parts: Vec<CompletedPart>,
}

impl FinishUploadParams {
    #[must_use]
    pub fn new(parts: Vec<CompletedPart>) -> FinishUploadParams {
        FinishUploadParams { parts }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UploadPartUrl {
    pub part_number: i32,
    pub url: String,
    pub offset: i32,
    pub length: i32,
}

impl UploadPartUrl {
    #[must_use]
    pub fn new(part_number: i32, url: String, offset: i32, length: i32) -> UploadPartUrl {
        UploadPartUrl {
            part_number,
            url,
            offset,
            length,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompletedPart {
    #[serde(rename = "ETag")]
    pub e_tag: String,
    #[serde(rename = "PartNumber")]
    pub part_number: i32,
}

impl CompletedPart {
    #[must_use]
    pub fn new(e_tag: String, part_number: i32) -> CompletedPart {
        CompletedPart { e_tag, part_number }
    }
}
