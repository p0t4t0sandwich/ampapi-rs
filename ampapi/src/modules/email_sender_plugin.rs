#![allow(dead_code, non_camel_case_types, non_snake_case, unused_imports, unused_mut)]
use crate::{AMPAPI, types::*};

use std::collections::HashMap;

use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// struct EmailSenderPlugin
#[derive(Debug, Clone)]
pub struct EmailSenderPlugin {
    pub ampapi: AMPAPI
}

impl EmailSenderPlugin {
	///new - Creates a new EmailSenderPlugin object
	pub fn new(ampapi: AMPAPI) -> EmailSenderPlugin {
		EmailSenderPlugin {
			ampapi
		}
	}

    /// TestSMTPSettings - 
    /// Name Description Optional
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn TestSMTPSettings(&mut self, ) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Task<ActionResult<Value>>>("EmailSenderPlugin/TestSMTPSettings".to_string(), args)
    }

}
