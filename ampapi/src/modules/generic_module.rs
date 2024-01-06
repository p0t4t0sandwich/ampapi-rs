#![allow(dead_code, non_camel_case_types, non_snake_case, unused_imports, unused_mut)]
use crate::{AMPAPI, types::*};

use std::collections::HashMap;

use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// struct GenericModule
#[derive(Debug, Clone)]
pub struct GenericModule {
    pub ampapi: AMPAPI
}

impl GenericModule {
	///new - Creates a new GenericModule object
	pub fn new(ampapi: AMPAPI) -> GenericModule {
		GenericModule {
			ampapi
		}
	}

    /// ImportConfig - 
    /// Name Description Optional
    /// * `param` filename String  False
    /// Return core::result::Result<HashMap<String, String>, reqwest::Error>
    pub fn ImportConfig(&mut self, filename: String) -> core::result::Result<HashMap<String, String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("filename".to_string(), filename.into());
        self.ampapi.api_call::<HashMap<String, String>>("GenericModule/ImportConfig".to_string(), args)
    }

}
