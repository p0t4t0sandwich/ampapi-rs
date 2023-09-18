#![allow(dead_code, non_camel_case_types, non_snake_case, unused_imports, unused_mut)]
use crate::{AMPAPI, types::*};

use std::collections::HashMap;

use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// struct RCONPlugin
#[derive(Debug, Clone)]
pub struct RCONPlugin {
    pub ampapi: AMPAPI
}

impl RCONPlugin {
	///new - Creates a new RCONPlugin object
	pub fn new(ampapi: AMPAPI) -> RCONPlugin {
		RCONPlugin {
			ampapi
		}
	}

    /// Dummy - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Dummy(&mut self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("RCONPlugin/Dummy".to_string(), args)
    }

}
