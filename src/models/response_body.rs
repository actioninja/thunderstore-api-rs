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
pub struct ResponseBody {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl ResponseBody {
    pub fn new(email: Option<String>, session_id: String, username: String) -> ResponseBody {
        ResponseBody {
            email,
            session_id,
            username,
        }
    }
}


