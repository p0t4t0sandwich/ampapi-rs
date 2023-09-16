use crate::{AMPAPI, types::*};

use std::collections::HashMap;

#[allow(unused_imports)]
use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandich

/// struct steamcmdplugin
#[derive(Debug, Clone)]
pub struct steamcmdplugin {
    pub ampapi: AMPAPI
}

#[allow(non_snake_case, dead_code, unused_mut)]
impl steamcmdplugin {
	///new - Creates a new steamcmdplugin object
	pub fn new(ampapi: AMPAPI) -> steamcmdplugin {
		steamcmdplugin {
			ampapi
		}
	}

    /// CancelSteamGuard - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn CancelSteamGuard(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("steamcmdplugin/CancelSteamGuard".to_string(), args)
    }

    /// SteamGuardCode - 
    /// Name Description Optional
    /// * `param` code String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn SteamGuardCode(&self, code: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("code".to_string(), code.into());
        self.ampapi.api_call::<Value>("steamcmdplugin/SteamGuardCode".to_string(), args)
    }

    /// SteamUsernamePassword - 
    /// Name Description Optional
    /// * `param` username String  False
    /// * `param` password String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn SteamUsernamePassword(&self, username: String, password: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("username".to_string(), username.into());
        args.insert("password".to_string(), password.into());
        self.ampapi.api_call::<Value>("steamcmdplugin/SteamUsernamePassword".to_string(), args)
    }

}
