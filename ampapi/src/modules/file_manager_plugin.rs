#[allow(unused_imports)]
use crate::{AMPAPI, types::*};

use std::collections::HashMap;

#[allow(unused_imports)]
use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// struct FileManagerPlugin
#[derive(Debug, Clone)]
pub struct FileManagerPlugin {
    pub ampapi: AMPAPI
}

#[allow(non_snake_case, dead_code, unused_mut)]
impl FileManagerPlugin {
	///new - Creates a new FileManagerPlugin object
	pub fn new(ampapi: AMPAPI) -> FileManagerPlugin {
		FileManagerPlugin {
			ampapi
		}
	}

    /// AppendFileChunk - 
    /// Name Description Optional
    /// * `param` Filename String  False
    /// * `param` Data String  False
    /// * `param` Delete bool  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn AppendFileChunk(&self, Filename: String, Data: String, Delete: bool) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Filename".to_string(), Filename.into());
        args.insert("Data".to_string(), Data.into());
        args.insert("Delete".to_string(), Delete.into());
        self.ampapi.api_call::<Value>("FileManagerPlugin/AppendFileChunk".to_string(), args)
    }

    /// CalculateFileMD5Sum - 
    /// Name Description Optional
    /// * `param` FilePath String  False
    /// Return core::result::Result<ActionResult<String>, reqwest::Error>
    pub fn CalculateFileMD5Sum(&self, FilePath: String) -> core::result::Result<ActionResult<String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("FilePath".to_string(), FilePath.into());
        self.ampapi.api_call::<ActionResult<String>>("FileManagerPlugin/CalculateFileMD5Sum".to_string(), args)
    }

    /// ChangeExclusion - 
    /// Name Description Optional
    /// * `param` ModifyPath String  False
    /// * `param` AsDirectory bool  False
    /// * `param` Exclude bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ChangeExclusion(&self, ModifyPath: String, AsDirectory: bool, Exclude: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ModifyPath".to_string(), ModifyPath.into());
        args.insert("AsDirectory".to_string(), AsDirectory.into());
        args.insert("Exclude".to_string(), Exclude.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/ChangeExclusion".to_string(), args)
    }

    /// CopyFile - 
    /// Name Description Optional
    /// * `param` Origin String  False
    /// * `param` TargetDirectory String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CopyFile(&self, Origin: String, TargetDirectory: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Origin".to_string(), Origin.into());
        args.insert("TargetDirectory".to_string(), TargetDirectory.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/CopyFile".to_string(), args)
    }

    /// CreateArchive - 
    /// Name Description Optional
    /// * `param` PathToArchive String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CreateArchive(&self, PathToArchive: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("PathToArchive".to_string(), PathToArchive.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/CreateArchive".to_string(), args)
    }

    /// CreateDirectory - 
    /// Name Description Optional
    /// * `param` NewPath String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CreateDirectory(&self, NewPath: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("NewPath".to_string(), NewPath.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/CreateDirectory".to_string(), args)
    }

    /// DownloadFileFromURL - 
    /// Name Description Optional
    /// * `param` Source URL  False
    /// * `param` TargetDirectory String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DownloadFileFromURL(&self, Source: URL, TargetDirectory: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Source".to_string(), Source.into());
        args.insert("TargetDirectory".to_string(), TargetDirectory.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/DownloadFileFromURL".to_string(), args)
    }

    /// Dummy - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Dummy(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("FileManagerPlugin/Dummy".to_string(), args)
    }

    /// EmptyTrash - 
    /// Name Description Optional
    /// * `param` TrashDirectoryName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn EmptyTrash(&self, TrashDirectoryName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TrashDirectoryName".to_string(), TrashDirectoryName.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/EmptyTrash".to_string(), args)
    }

    /// ExtractArchive - 
    /// Name Description Optional
    /// * `param` ArchivePath String  False
    /// * `param` DestinationPath String  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ExtractArchive(&self, ArchivePath: String, DestinationPath: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ArchivePath".to_string(), ArchivePath.into());
        args.insert("DestinationPath".to_string(), DestinationPath.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/ExtractArchive".to_string(), args)
    }

    /// GetDirectoryListing - 
    /// Name Description Optional
    /// * `param` Dir String  False
    /// Return core::result::Result<Result<HashMap<String, Value>>, reqwest::Error>
    pub fn GetDirectoryListing(&self, Dir: String) -> core::result::Result<Result<HashMap<String, Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Dir".to_string(), Dir.into());
        self.ampapi.api_call::<Result<HashMap<String, Value>>>("FileManagerPlugin/GetDirectoryListing".to_string(), args)
    }

    /// GetFileChunk - 
    /// Name Description Optional
    /// * `param` Filename String  False
    /// * `param` Position i64  False
    /// * `param` Length i32  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetFileChunk(&self, Filename: String, Position: i64, Length: i32) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Filename".to_string(), Filename.into());
        args.insert("Position".to_string(), Position.into());
        args.insert("Length".to_string(), Length.into());
        self.ampapi.api_call::<Value>("FileManagerPlugin/GetFileChunk".to_string(), args)
    }

    /// ReadFileChunk - 
    /// Name Description Optional
    /// * `param` Filename String  False
    /// * `param` Offset i64  False
    /// * `param` ChunkSize i64  True
    /// Return core::result::Result<ActionResult<String>, reqwest::Error>
    pub fn ReadFileChunk(&self, Filename: String, Offset: i64, ChunkSize: i64) -> core::result::Result<ActionResult<String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Filename".to_string(), Filename.into());
        args.insert("Offset".to_string(), Offset.into());
        args.insert("ChunkSize".to_string(), ChunkSize.into());
        self.ampapi.api_call::<ActionResult<String>>("FileManagerPlugin/ReadFileChunk".to_string(), args)
    }

    /// RenameDirectory - The name component of the new directory (not the full path)
    /// Name Description Optional
    /// * `param` oldDirectory String The full path to the old directory False
    /// * `param` NewDirectoryName String The name component of the new directory (not the full path) False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RenameDirectory(&self, oldDirectory: String, NewDirectoryName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("oldDirectory".to_string(), oldDirectory.into());
        args.insert("NewDirectoryName".to_string(), NewDirectoryName.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/RenameDirectory".to_string(), args)
    }

    /// RenameFile - 
    /// Name Description Optional
    /// * `param` Filename String  False
    /// * `param` NewFilename String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RenameFile(&self, Filename: String, NewFilename: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Filename".to_string(), Filename.into());
        args.insert("NewFilename".to_string(), NewFilename.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/RenameFile".to_string(), args)
    }

    /// TrashDirectory - 
    /// Name Description Optional
    /// * `param` DirectoryName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn TrashDirectory(&self, DirectoryName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("DirectoryName".to_string(), DirectoryName.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/TrashDirectory".to_string(), args)
    }

    /// TrashFile - 
    /// Name Description Optional
    /// * `param` Filename String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn TrashFile(&self, Filename: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Filename".to_string(), Filename.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/TrashFile".to_string(), args)
    }

    /// WriteFileChunk - 
    /// Name Description Optional
    /// * `param` Filename String  False
    /// * `param` Data String  False
    /// * `param` Offset i64  False
    /// * `param` FinalChunk bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn WriteFileChunk(&self, Filename: String, Data: String, Offset: i64, FinalChunk: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Filename".to_string(), Filename.into());
        args.insert("Data".to_string(), Data.into());
        args.insert("Offset".to_string(), Offset.into());
        args.insert("FinalChunk".to_string(), FinalChunk.into());
        self.ampapi.api_call::<ActionResult<Value>>("FileManagerPlugin/WriteFileChunk".to_string(), args)
    }

}
