use crate::{AMPAPI, types::*};

use std::collections::HashMap;

#[allow(unused_imports)]
use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandich

/// struct MinecraftModule
#[derive(Debug, Clone)]
pub struct MinecraftModule {
    pub ampapi: AMPAPI
}

#[allow(non_snake_case, dead_code, unused_mut)]
impl MinecraftModule {
	///new - Creates a new MinecraftModule object
	pub fn new(ampapi: AMPAPI) -> MinecraftModule {
		MinecraftModule {
			ampapi
		}
	}

    /// AcceptEULA - 
    /// Name Description Optional
    /// Return core::result::Result<bool, reqwest::Error>
    pub fn AcceptEULA(&self, ) -> core::result::Result<bool, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<bool>("MinecraftModule/AcceptEULA".to_string(), args)
    }

    /// AddOPEntry - 
    /// Name Description Optional
    /// * `param` UserOrUUID String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn AddOPEntry(&self, UserOrUUID: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("UserOrUUID".to_string(), UserOrUUID.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("MinecraftModule/AddOPEntry".to_string(), args)
    }

    /// AddToWhitelist - 
    /// Name Description Optional
    /// * `param` UserOrUUID String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn AddToWhitelist(&self, UserOrUUID: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("UserOrUUID".to_string(), UserOrUUID.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("MinecraftModule/AddToWhitelist".to_string(), args)
    }

    /// BanUserByID - 
    /// Name Description Optional
    /// * `param` ID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BanUserByID(&self, ID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ID".to_string(), ID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/BanUserByID".to_string(), args)
    }

    /// BukGetCategories - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetCategories(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetCategories".to_string(), args)
    }

    /// BukGetInstallUpdatePlugin - 
    /// Name Description Optional
    /// * `param` pluginId i32  False
    /// Return core::result::Result<Task<RunningTask>, reqwest::Error>
    pub fn BukGetInstallUpdatePlugin(&self, pluginId: i32) -> core::result::Result<Task<RunningTask>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("pluginId".to_string(), pluginId.into());
        self.ampapi.api_call::<Task<RunningTask>>("MinecraftModule/BukGetInstallUpdatePlugin".to_string(), args)
    }

    /// BukGetInstalledPlugins - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetInstalledPlugins(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetInstalledPlugins".to_string(), args)
    }

    /// BukGetPluginInfo - 
    /// Name Description Optional
    /// * `param` PluginId i32  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetPluginInfo(&self, PluginId: i32) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("PluginId".to_string(), PluginId.into());
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetPluginInfo".to_string(), args)
    }

    /// BukGetPluginsForCategory - 
    /// Name Description Optional
    /// * `param` CategoryId String  False
    /// * `param` PageNumber i32  False
    /// * `param` PageSize i32  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetPluginsForCategory(&self, CategoryId: String, PageNumber: i32, PageSize: i32) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("CategoryId".to_string(), CategoryId.into());
        args.insert("PageNumber".to_string(), PageNumber.into());
        args.insert("PageSize".to_string(), PageSize.into());
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetPluginsForCategory".to_string(), args)
    }

    /// BukGetPopularPlugins - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetPopularPlugins(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetPopularPlugins".to_string(), args)
    }

    /// BukGetRemovePlugin - 
    /// Name Description Optional
    /// * `param` PluginId i32  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetRemovePlugin(&self, PluginId: i32) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("PluginId".to_string(), PluginId.into());
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetRemovePlugin".to_string(), args)
    }

    /// BukGetSearch - 
    /// Name Description Optional
    /// * `param` Query String  False
    /// * `param` PageNumber i32  False
    /// * `param` PageSize i32  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn BukGetSearch(&self, Query: String, PageNumber: i32, PageSize: i32) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Query".to_string(), Query.into());
        args.insert("PageNumber".to_string(), PageNumber.into());
        args.insert("PageSize".to_string(), PageSize.into());
        self.ampapi.api_call::<Value>("MinecraftModule/BukGetSearch".to_string(), args)
    }

    /// ClearInventoryByID - 
    /// Name Description Optional
    /// * `param` ID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn ClearInventoryByID(&self, ID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ID".to_string(), ID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/ClearInventoryByID".to_string(), args)
    }

    /// GetFailureReason - 
    /// Name Description Optional
    /// Return core::result::Result<String, reqwest::Error>
    pub fn GetFailureReason(&self, ) -> core::result::Result<String, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<String>("MinecraftModule/GetFailureReason".to_string(), args)
    }

    /// GetHeadByUUID - 
    /// Name Description Optional
    /// * `param` id String  False
    /// Return core::result::Result<String, reqwest::Error>
    pub fn GetHeadByUUID(&self, id: String) -> core::result::Result<String, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("id".to_string(), id.into());
        self.ampapi.api_call::<String>("MinecraftModule/GetHeadByUUID".to_string(), args)
    }

    /// GetOPWhitelist - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetOPWhitelist(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("MinecraftModule/GetOPWhitelist".to_string(), args)
    }

    /// GetWhitelist - 
    /// Name Description Optional
    /// Return core::result::Result<Result<HashMap<String, Value>>, reqwest::Error>
    pub fn GetWhitelist(&self, ) -> core::result::Result<Result<HashMap<String, Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<HashMap<String, Value>>>("MinecraftModule/GetWhitelist".to_string(), args)
    }

    /// KickUserByID - 
    /// Name Description Optional
    /// * `param` ID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn KickUserByID(&self, ID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ID".to_string(), ID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/KickUserByID".to_string(), args)
    }

    /// KillByID - 
    /// Name Description Optional
    /// * `param` ID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn KillByID(&self, ID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ID".to_string(), ID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/KillByID".to_string(), args)
    }

    /// LoadOPList - 
    /// Name Description Optional
    /// Return core::result::Result<Result<HashMap<String, Value>>, reqwest::Error>
    pub fn LoadOPList(&self, ) -> core::result::Result<Result<HashMap<String, Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<HashMap<String, Value>>>("MinecraftModule/LoadOPList".to_string(), args)
    }

    /// RemoveOPEntry - 
    /// Name Description Optional
    /// * `param` UserOrUUID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn RemoveOPEntry(&self, UserOrUUID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("UserOrUUID".to_string(), UserOrUUID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/RemoveOPEntry".to_string(), args)
    }

    /// RemoveWhitelistEntry - 
    /// Name Description Optional
    /// * `param` UserOrUUID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn RemoveWhitelistEntry(&self, UserOrUUID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("UserOrUUID".to_string(), UserOrUUID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/RemoveWhitelistEntry".to_string(), args)
    }

    /// SmiteByID - 
    /// Name Description Optional
    /// * `param` ID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn SmiteByID(&self, ID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ID".to_string(), ID.into());
        self.ampapi.api_call::<Value>("MinecraftModule/SmiteByID".to_string(), args)
    }

}
