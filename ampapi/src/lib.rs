#![crate_name = "ampapi"]

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::de;

use serde_json::Value;

use reqwest::blocking;

mod types;
use crate::types::LoginResult;

mod modules;


/// AMPAPI - Struct for interacting with the AMP API
#[derive(Debug, Clone)]
pub struct AMPAPI {
    /// base_uri - The base URI of the AMP instance
    pub base_uri: String,
    /// data_source - The data source URI of the AMP instance
    pub data_source: String,
    /// username - The username to use for authentication
    pub username: String,
    /// password - The password to use for authentication
    pub password: String,
    /// remember_me_token - The remember me token to use for authentication
    pub remember_me_token: String,
    /// session_id - The session ID to use for authentication
    pub session_id: String,
    /// last_api_call - Time of the last API call
    last_api_call: i64,
    /// relog_interval - The interval to relog at
    relog_interval: i64,
}

/// AMPAPI methods
impl AMPAPI {
    /// AMPAPI.new - Create a new AMPAPI struct
    /// * `base_uri` - The base URI of the AMP instance
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///   * Optional if using a remember me token
    /// * `remember_me_token` - The remember me token to use for authentication
    ///   * Optional
    /// * `session_id` - The session ID to use for authentication
    ///   * Optional, will be instantiated on login
    /// Returns AMPAPI
    pub fn new(base_uri: String, username: String, password: String, remember_me_token: String, session_id: String) -> AMPAPI {
        let mut new_base_uri = base_uri.clone();
        if (new_base_uri.chars().nth(base_uri.len()-1).unwrap()) != '/' {
            new_base_uri.push('/');
        }

        AMPAPI {
            base_uri: new_base_uri.clone(),
            data_source: new_base_uri + "API/",
            username,
            password,
            remember_me_token,
            session_id,
            last_api_call: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
            relog_interval: 1000*60*5,
        }
    }

    /// AMPAPI.api_call - This function takes an endpoint and a map of arguments and returns the response
    /// * `endpoint` - The endpoint to call
    /// * `args` - A map of arguments to pass to the endpoint
    /// Returns Result<T, reqwest::Error>
    pub fn api_call<T: de::DeserializeOwned>(&mut self, endpoint: String, args: HashMap<String, Value>) -> Result<T, reqwest::Error> {
        // Check the last API call time, and if it's been more than the relog interval, relog.
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        if now - self.last_api_call > self.relog_interval {
            self.last_api_call = now;
            let _ = self.login();
        } else {
            self.last_api_call = now;
        }
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("SESSIONID".to_string(), Value::String(self.session_id.to_string()));
        for (key, value) in args.iter() {
            map.insert(key.to_string(), value.clone());
        }

        let client = blocking::Client::new();
        let url = format!("{}{}", self.data_source, endpoint);
        let response = client.post(url)
            .json(&map)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", "ampapi-rust/0.1.1")
            .send()?
            .json::<T>()?;
        Ok(response)
    }

    /// AMPAPI.login - Simplified login function
    /// Returns Result<LoginResult, reqwest::Error>
    pub fn login(&mut self) -> Result<LoginResult, reqwest::Error> {
        let mut args = HashMap::new();
        args.insert("username".to_string(), Value::String(self.username.clone()));
        args.insert("password".to_string(), Value::String("".to_string()));
        args.insert("token".to_string(), Value::String(self.remember_me_token.clone()));
        args.insert("rememberMe".to_string(), Value::Bool(true));

        // If remember me token is empty, use the password.
        if self.remember_me_token == "" {
            args.insert("password".to_string(), Value::String(self.password.clone()));
        }

        self.api_call::<LoginResult>("Core/Login".to_string(), args)
    }
}
