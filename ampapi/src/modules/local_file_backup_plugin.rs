#![allow(dead_code, non_camel_case_types, non_snake_case, unused_imports, unused_mut)]
use crate::{AMPAPI, types::*};

use std::collections::HashMap;

use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// struct LocalFileBackupPlugin
#[derive(Debug, Clone)]
pub struct LocalFileBackupPlugin {
    pub ampapi: AMPAPI
}

impl LocalFileBackupPlugin {
	///new - Creates a new LocalFileBackupPlugin object
	pub fn new(ampapi: AMPAPI) -> LocalFileBackupPlugin {
		LocalFileBackupPlugin {
			ampapi
		}
	}

    /// DeleteFromS3 - 
    /// Name Description Optional
    /// * `param` BackupId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DeleteFromS3(&mut self, BackupId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("BackupId".to_string(), BackupId.into());
        self.ampapi.api_call::<ActionResult<Value>>("LocalFileBackupPlugin/DeleteFromS3".to_string(), args)
    }

    /// DeleteLocalBackup - 
    /// Name Description Optional
    /// * `param` BackupId UUID  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn DeleteLocalBackup(&mut self, BackupId: UUID) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("BackupId".to_string(), BackupId.into());
        self.ampapi.api_call::<Value>("LocalFileBackupPlugin/DeleteLocalBackup".to_string(), args)
    }

    /// DownloadFromS3 - 
    /// Name Description Optional
    /// * `param` BackupId UUID  False
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn DownloadFromS3(&mut self, BackupId: UUID) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("BackupId".to_string(), BackupId.into());
        self.ampapi.api_call::<RunningTask>("LocalFileBackupPlugin/DownloadFromS3".to_string(), args)
    }

    /// GetBackups - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetBackups(&mut self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("LocalFileBackupPlugin/GetBackups".to_string(), args)
    }

    /// RestoreBackup - 
    /// Name Description Optional
    /// * `param` BackupId UUID  False
    /// * `param` DeleteExistingData bool  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RestoreBackup(&mut self, BackupId: UUID, DeleteExistingData: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("BackupId".to_string(), BackupId.into());
        args.insert("DeleteExistingData".to_string(), DeleteExistingData.into());
        self.ampapi.api_call::<ActionResult<Value>>("LocalFileBackupPlugin/RestoreBackup".to_string(), args)
    }

    /// SetBackupSticky - 
    /// Name Description Optional
    /// * `param` BackupId UUID  False
    /// * `param` Sticky bool  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn SetBackupSticky(&mut self, BackupId: UUID, Sticky: bool) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("BackupId".to_string(), BackupId.into());
        args.insert("Sticky".to_string(), Sticky.into());
        self.ampapi.api_call::<Value>("LocalFileBackupPlugin/SetBackupSticky".to_string(), args)
    }

    /// TakeBackup - 
    /// Name Description Optional
    /// * `param` Title String  False
    /// * `param` Description String  False
    /// * `param` Sticky bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn TakeBackup(&mut self, Title: String, Description: String, Sticky: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Title".to_string(), Title.into());
        args.insert("Description".to_string(), Description.into());
        args.insert("Sticky".to_string(), Sticky.into());
        self.ampapi.api_call::<ActionResult<Value>>("LocalFileBackupPlugin/TakeBackup".to_string(), args)
    }

    /// UploadToS3 - 
    /// Name Description Optional
    /// * `param` BackupId UUID  False
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn UploadToS3(&mut self, BackupId: UUID) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("BackupId".to_string(), BackupId.into());
        self.ampapi.api_call::<RunningTask>("LocalFileBackupPlugin/UploadToS3".to_string(), args)
    }

}
