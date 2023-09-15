use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AMPAPI {
    pub base_uri: String,
    pub data_source: String,
    pub username: String,
    pub password: String,
    pub remember_me_token: String,
    pub session_id: String,
}

impl AMPAPI {
    pub fn new(base_uri: String, data_source: String, username: String, password: String, remember_me_token: String, session_id: String) -> AMPAPI {
        AMPAPI {
            base_uri,
            data_source,
            username,
            password,
            remember_me_token,
            session_id,
        }
    }
}
