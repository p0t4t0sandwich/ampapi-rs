#![allow(dead_code, non_camel_case_types, non_snake_case, unused_imports, unused_mut)]
use crate::{AMPAPI, types::*};

use std::collections::HashMap;

use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// struct ADSModule
#[derive(Debug, Clone)]
pub struct ADSModule {
    pub ampapi: AMPAPI
}

impl ADSModule {
	///new - Creates a new ADSModule object
	pub fn new(ampapi: AMPAPI) -> ADSModule {
		ADSModule {
			ampapi
		}
	}

    /// AddDatastore - 
    /// Name Description Optional
    /// * `param` newDatastore Value  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn AddDatastore(&mut self, newDatastore: Value) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("newDatastore".to_string(), newDatastore.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/AddDatastore".to_string(), args)
    }

    /// ApplyInstanceConfiguration - 
    /// Name Description Optional
    /// * `param` InstanceID UUID  False
    /// * `param` Args Map<String, Value>  False
    /// * `param` RebuildConfiguration bool  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ApplyInstanceConfiguration(&mut self, InstanceID: UUID, Args: Map<String, Value>, RebuildConfiguration: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceID".to_string(), InstanceID.into());
        args.insert("Args".to_string(), Args.into());
        args.insert("RebuildConfiguration".to_string(), RebuildConfiguration.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/ApplyInstanceConfiguration".to_string(), args)
    }

    /// ApplyTemplate - 
    /// Name Description Optional
    /// * `param` InstanceID UUID  False
    /// * `param` TemplateID i32  False
    /// * `param` NewFriendlyName String  True
    /// * `param` Secret String  True
    /// * `param` RestartIfPreviouslyRunning bool  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ApplyTemplate(&mut self, InstanceID: UUID, TemplateID: i32, NewFriendlyName: String, Secret: String, RestartIfPreviouslyRunning: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceID".to_string(), InstanceID.into());
        args.insert("TemplateID".to_string(), TemplateID.into());
        args.insert("NewFriendlyName".to_string(), NewFriendlyName.into());
        args.insert("Secret".to_string(), Secret.into());
        args.insert("RestartIfPreviouslyRunning".to_string(), RestartIfPreviouslyRunning.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/ApplyTemplate".to_string(), args)
    }

    /// AttachADS - 
    /// Name Description Optional
    /// * `param` Friendly String  False
    /// * `param` IsHTTPS bool  False
    /// * `param` Host String  False
    /// * `param` Port i32  False
    /// * `param` InstanceID UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn AttachADS(&mut self, Friendly: String, IsHTTPS: bool, Host: String, Port: i32, InstanceID: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Friendly".to_string(), Friendly.into());
        args.insert("IsHTTPS".to_string(), IsHTTPS.into());
        args.insert("Host".to_string(), Host.into());
        args.insert("Port".to_string(), Port.into());
        args.insert("InstanceID".to_string(), InstanceID.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/AttachADS".to_string(), args)
    }

    /// CloneTemplate - 
    /// Name Description Optional
    /// * `param` Id i32  False
    /// * `param` NewName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CloneTemplate(&mut self, Id: i32, NewName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        args.insert("NewName".to_string(), NewName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/CloneTemplate".to_string(), args)
    }

    /// ConvertToManaged - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ConvertToManaged(&mut self, InstanceName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/ConvertToManaged".to_string(), args)
    }

    /// CreateDeploymentTemplate - 
    /// Name Description Optional
    /// * `param` Name String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CreateDeploymentTemplate(&mut self, Name: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Name".to_string(), Name.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/CreateDeploymentTemplate".to_string(), args)
    }

    /// CreateInstance - 
    /// Name Description Optional
    /// * `param` TargetADSInstance UUID  False
    /// * `param` NewInstanceId UUID  False
    /// * `param` Module String  False
    /// * `param` InstanceName String  False
    /// * `param` FriendlyName String  False
    /// * `param` IPBinding String  False
    /// * `param` PortNumber i32  False
    /// * `param` AdminUsername String  False
    /// * `param` AdminPassword String  False
    /// * `param` ProvisionSettings Map<String, Value>  False
    /// * `param` AutoConfigure bool When enabled, all settings other than the Module, Target and FriendlyName are ignored and replaced with automatically generated values. True
    /// * `param` PostCreate Value  True
    /// * `param` StartOnBoot bool  True
    /// * `param` DisplayImageSource String  True
    /// * `param` TargetDatastore i32  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CreateInstance(&mut self, TargetADSInstance: UUID, NewInstanceId: UUID, Module: String, InstanceName: String, FriendlyName: String, IPBinding: String, PortNumber: i32, AdminUsername: String, AdminPassword: String, ProvisionSettings: Map<String, Value>, AutoConfigure: bool, PostCreate: Value, StartOnBoot: bool, DisplayImageSource: String, TargetDatastore: i32) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TargetADSInstance".to_string(), TargetADSInstance.into());
        args.insert("NewInstanceId".to_string(), NewInstanceId.into());
        args.insert("Module".to_string(), Module.into());
        args.insert("InstanceName".to_string(), InstanceName.into());
        args.insert("FriendlyName".to_string(), FriendlyName.into());
        args.insert("IPBinding".to_string(), IPBinding.into());
        args.insert("PortNumber".to_string(), PortNumber.into());
        args.insert("AdminUsername".to_string(), AdminUsername.into());
        args.insert("AdminPassword".to_string(), AdminPassword.into());
        args.insert("ProvisionSettings".to_string(), ProvisionSettings.into());
        args.insert("AutoConfigure".to_string(), AutoConfigure.into());
        args.insert("PostCreate".to_string(), PostCreate.into());
        args.insert("StartOnBoot".to_string(), StartOnBoot.into());
        args.insert("DisplayImageSource".to_string(), DisplayImageSource.into());
        args.insert("TargetDatastore".to_string(), TargetDatastore.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/CreateInstance".to_string(), args)
    }

    /// CreateLocalInstance - 
    /// Name Description Optional
    /// * `param` Instance Value  False
    /// * `param` PostCreate Value  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CreateLocalInstance(&mut self, Instance: Value, PostCreate: Value) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Instance".to_string(), Instance.into());
        args.insert("PostCreate".to_string(), PostCreate.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/CreateLocalInstance".to_string(), args)
    }

    /// DeleteDatastore - 
    /// Name Description Optional
    /// * `param` id i32  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DeleteDatastore(&mut self, id: i32) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("id".to_string(), id.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/DeleteDatastore".to_string(), args)
    }

    /// DeleteDeploymentTemplate - 
    /// Name Description Optional
    /// * `param` Id i32  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DeleteDeploymentTemplate(&mut self, Id: i32) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/DeleteDeploymentTemplate".to_string(), args)
    }

    /// DeleteInstance - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn DeleteInstance(&mut self, InstanceName: String) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<RunningTask>("ADSModule/DeleteInstance".to_string(), args)
    }

    /// DeleteInstanceUsers - 
    /// Name Description Optional
    /// * `param` InstanceId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DeleteInstanceUsers(&mut self, InstanceId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/DeleteInstanceUsers".to_string(), args)
    }

    /// DeployTemplate - A dictionary of setting nodes and values to create the new instance with. Identical in function to the provisioning arguments in the template itself.
    /// Name Description Optional
    /// * `param` TemplateID i32 The ID of the template to be deployed, as per the Template Management UI in AMP itself. False
    /// * `param` NewUsername String If specified, AMP will create a new user with this name for this instance. Must be unique. If this user already exists, this will be ignored but the new instance will be assigned to this user. True
    /// * `param` NewPassword String If 'NewUsername' is specified and the user doesn't already exist, the password that will be assigned to this user. True
    /// * `param` NewEmail String If 'NewUsername' is specified and the user doesn't already exist, the email address that will be assigned to this user. True
    /// * `param` RequiredTags Vec<String> If specified, AMP will only deploy this template to targets that have every single 'tag' specified in their target configuration. You can adjust this via the controller by clicking 'Edit' on the target settings. True
    /// * `param` Tag String Unrelated to RequiredTags. This is to uniquely identify this instance to your own systems. It may be something like an order ID or service ID so you can find the associated instance again at a later time. If 'UseTagAsInstanceName' is enabled, then this will also be used as the instance name for the created instance - but it must be unique. True
    /// * `param` FriendlyName String A friendly name for this instance. If left blank, AMP will generate one for you. True
    /// * `param` Secret String Must be a non-empty strong in order to get a callback on deployment state change. This secret will be passed back to you in the callback so you can verify the request. True
    /// * `param` PostCreate Value 0: Do nothing, 1: Start instance only, 2: Start instance and update application, 3: Full application startup. True
    /// * `param` ExtraProvisionSettings Map<String, Value> A dictionary of setting nodes and values to create the new instance with. Identical in function to the provisioning arguments in the template itself. True
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn DeployTemplate(&mut self, TemplateID: i32, NewUsername: String, NewPassword: String, NewEmail: String, RequiredTags: Vec<String>, Tag: String, FriendlyName: String, Secret: String, PostCreate: Value, ExtraProvisionSettings: Map<String, Value>) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TemplateID".to_string(), TemplateID.into());
        args.insert("NewUsername".to_string(), NewUsername.into());
        args.insert("NewPassword".to_string(), NewPassword.into());
        args.insert("NewEmail".to_string(), NewEmail.into());
        args.insert("RequiredTags".to_string(), RequiredTags.into());
        args.insert("Tag".to_string(), Tag.into());
        args.insert("FriendlyName".to_string(), FriendlyName.into());
        args.insert("Secret".to_string(), Secret.into());
        args.insert("PostCreate".to_string(), PostCreate.into());
        args.insert("ExtraProvisionSettings".to_string(), ExtraProvisionSettings.into());
        self.ampapi.api_call::<RunningTask>("ADSModule/DeployTemplate".to_string(), args)
    }

    /// DetatchTarget - 
    /// Name Description Optional
    /// * `param` Id UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DetatchTarget(&mut self, Id: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/DetatchTarget".to_string(), args)
    }

    /// ExtractEverywhere - 
    /// Name Description Optional
    /// * `param` SourceArchive String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ExtractEverywhere(&mut self, SourceArchive: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("SourceArchive".to_string(), SourceArchive.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/ExtractEverywhere".to_string(), args)
    }

    /// GetApplicationEndpoints - 
    /// Name Description Optional
    /// * `param` instanceId UUID  False
    /// Return core::result::Result<Vec<EndpointInfo>, reqwest::Error>
    pub fn GetApplicationEndpoints(&mut self, instanceId: UUID) -> core::result::Result<Vec<EndpointInfo>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("instanceId".to_string(), instanceId.into());
        self.ampapi.api_call::<Vec<EndpointInfo>>("ADSModule/GetApplicationEndpoints".to_string(), args)
    }

    /// GetDatastore - 
    /// Name Description Optional
    /// * `param` id i32  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetDatastore(&mut self, id: i32) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("id".to_string(), id.into());
        self.ampapi.api_call::<Value>("ADSModule/GetDatastore".to_string(), args)
    }

    /// GetDatastoreInstances - 
    /// Name Description Optional
    /// * `param` datastoreId i32  False
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetDatastoreInstances(&mut self, datastoreId: i32) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("datastoreId".to_string(), datastoreId.into());
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetDatastoreInstances".to_string(), args)
    }

    /// GetDatastores - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetDatastores(&mut self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetDatastores".to_string(), args)
    }

    /// GetDeploymentTemplates - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetDeploymentTemplates(&mut self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetDeploymentTemplates".to_string(), args)
    }

    /// GetGroup - 
    /// Name Description Optional
    /// * `param` GroupId UUID  False
    /// Return core::result::Result<IADSInstance, reqwest::Error>
    pub fn GetGroup(&mut self, GroupId: UUID) -> core::result::Result<IADSInstance, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("GroupId".to_string(), GroupId.into());
        self.ampapi.api_call::<IADSInstance>("ADSModule/GetGroup".to_string(), args)
    }

    /// GetInstance - 
    /// Name Description Optional
    /// * `param` InstanceId UUID  False
    /// Return core::result::Result<Instance, reqwest::Error>
    pub fn GetInstance(&mut self, InstanceId: UUID) -> core::result::Result<Instance, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        self.ampapi.api_call::<Instance>("ADSModule/GetInstance".to_string(), args)
    }

    /// GetInstanceNetworkInfo - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetInstanceNetworkInfo(&mut self, InstanceName: String) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetInstanceNetworkInfo".to_string(), args)
    }

    /// GetInstanceStatuses - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetInstanceStatuses(&mut self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetInstanceStatuses".to_string(), args)
    }

    /// GetInstances - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<IADSInstance>, reqwest::Error>
    pub fn GetInstances(&mut self, ) -> core::result::Result<Vec<IADSInstance>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<IADSInstance>>("ADSModule/GetInstances".to_string(), args)
    }

    /// GetLocalInstances - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetLocalInstances(&mut self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetLocalInstances".to_string(), args)
    }

    /// GetProvisionArguments - 
    /// Name Description Optional
    /// * `param` ModuleName String  False
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetProvisionArguments(&mut self, ModuleName: String) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ModuleName".to_string(), ModuleName.into());
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetProvisionArguments".to_string(), args)
    }

    /// GetProvisionFitness - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetProvisionFitness(&mut self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("ADSModule/GetProvisionFitness".to_string(), args)
    }

    /// GetSupportedApplications - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetSupportedApplications(&mut self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("ADSModule/GetSupportedApplications".to_string(), args)
    }

    /// GetTargetInfo - 
    /// Name Description Optional
    /// Return core::result::Result<RemoteTargetInfo, reqwest::Error>
    pub fn GetTargetInfo(&mut self, ) -> core::result::Result<RemoteTargetInfo, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<RemoteTargetInfo>("ADSModule/GetTargetInfo".to_string(), args)
    }

    /// HandoutInstanceConfigs - 
    /// Name Description Optional
    /// * `param` ForModule String  False
    /// * `param` SettingNode String  False
    /// * `param` Values Vec<String>  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn HandoutInstanceConfigs(&mut self, ForModule: String, SettingNode: String, Values: Vec<String>) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ForModule".to_string(), ForModule.into());
        args.insert("SettingNode".to_string(), SettingNode.into());
        args.insert("Values".to_string(), Values.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/HandoutInstanceConfigs".to_string(), args)
    }

    /// ManageInstance - 
    /// Name Description Optional
    /// * `param` InstanceId UUID  False
    /// Return core::result::Result<ActionResult<String>, reqwest::Error>
    pub fn ManageInstance(&mut self, InstanceId: UUID) -> core::result::Result<ActionResult<String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        self.ampapi.api_call::<ActionResult<String>>("ADSModule/ManageInstance".to_string(), args)
    }

    /// ModifyCustomFirewallRule - 
    /// Name Description Optional
    /// * `param` instanceId UUID  False
    /// * `param` PortNumber i32  False
    /// * `param` Range i32  False
    /// * `param` Protocol Value  False
    /// * `param` Description String  False
    /// * `param` Open bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ModifyCustomFirewallRule(&mut self, instanceId: UUID, PortNumber: i32, Range: i32, Protocol: Value, Description: String, Open: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("instanceId".to_string(), instanceId.into());
        args.insert("PortNumber".to_string(), PortNumber.into());
        args.insert("Range".to_string(), Range.into());
        args.insert("Protocol".to_string(), Protocol.into());
        args.insert("Description".to_string(), Description.into());
        args.insert("Open".to_string(), Open.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/ModifyCustomFirewallRule".to_string(), args)
    }

    /// MoveInstanceDatastore - 
    /// Name Description Optional
    /// * `param` instanceId UUID  False
    /// * `param` datastoreId i32  False
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn MoveInstanceDatastore(&mut self, instanceId: UUID, datastoreId: i32) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("instanceId".to_string(), instanceId.into());
        args.insert("datastoreId".to_string(), datastoreId.into());
        self.ampapi.api_call::<RunningTask>("ADSModule/MoveInstanceDatastore".to_string(), args)
    }

    /// ReactivateLocalInstances - 
    /// Name Description Optional
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn ReactivateLocalInstances(&mut self, ) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<RunningTask>("ADSModule/ReactivateLocalInstances".to_string(), args)
    }

    /// RefreshAppCache - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn RefreshAppCache(&mut self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("ADSModule/RefreshAppCache".to_string(), args)
    }

    /// RefreshGroup - 
    /// Name Description Optional
    /// * `param` GroupId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RefreshGroup(&mut self, GroupId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("GroupId".to_string(), GroupId.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/RefreshGroup".to_string(), args)
    }

    /// RefreshInstanceConfig - 
    /// Name Description Optional
    /// * `param` InstanceId String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RefreshInstanceConfig(&mut self, InstanceId: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/RefreshInstanceConfig".to_string(), args)
    }

    /// RefreshRemoteConfigStores - 
    /// Name Description Optional
    /// * `param` force bool  True
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn RefreshRemoteConfigStores(&mut self, force: bool) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("force".to_string(), force.into());
        self.ampapi.api_call::<Value>("ADSModule/RefreshRemoteConfigStores".to_string(), args)
    }

    /// RegisterTarget - 
    /// Name Description Optional
    /// * `param` controllerUrl String  False
    /// * `param` myUrl String  False
    /// * `param` username String  False
    /// * `param` password String  False
    /// * `param` twoFactorToken String  False
    /// * `param` friendlyName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RegisterTarget(&mut self, controllerUrl: String, myUrl: String, username: String, password: String, twoFactorToken: String, friendlyName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("controllerUrl".to_string(), controllerUrl.into());
        args.insert("myUrl".to_string(), myUrl.into());
        args.insert("username".to_string(), username.into());
        args.insert("password".to_string(), password.into());
        args.insert("twoFactorToken".to_string(), twoFactorToken.into());
        args.insert("friendlyName".to_string(), friendlyName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/RegisterTarget".to_string(), args)
    }

    /// RepairDatastore - 
    /// Name Description Optional
    /// * `param` id i32  False
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn RepairDatastore(&mut self, id: i32) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("id".to_string(), id.into());
        self.ampapi.api_call::<RunningTask>("ADSModule/RepairDatastore".to_string(), args)
    }

    /// RequestDatastoreSizeCalculation - 
    /// Name Description Optional
    /// * `param` datastoreId i32  False
    /// Return core::result::Result<RunningTask, reqwest::Error>
    pub fn RequestDatastoreSizeCalculation(&mut self, datastoreId: i32) -> core::result::Result<RunningTask, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("datastoreId".to_string(), datastoreId.into());
        self.ampapi.api_call::<RunningTask>("ADSModule/RequestDatastoreSizeCalculation".to_string(), args)
    }

    /// RestartInstance - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RestartInstance(&mut self, InstanceName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/RestartInstance".to_string(), args)
    }

    /// Servers - 
    /// Name Description Optional
    /// * `param` id String  False
    /// * `param` REQ_RAWJSON String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Servers(&mut self, id: String, REQ_RAWJSON: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("id".to_string(), id.into());
        args.insert("REQ_RAWJSON".to_string(), REQ_RAWJSON.into());
        self.ampapi.api_call::<Value>("ADSModule/Servers".to_string(), args)
    }

    /// SetInstanceConfig - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// * `param` SettingNode String  False
    /// * `param` Value String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn SetInstanceConfig(&mut self, InstanceName: String, SettingNode: String, Value: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        args.insert("SettingNode".to_string(), SettingNode.into());
        args.insert("Value".to_string(), Value.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/SetInstanceConfig".to_string(), args)
    }

    /// SetInstanceNetworkInfo - 
    /// Name Description Optional
    /// * `param` InstanceId UUID  False
    /// * `param` PortMappings Map<String, Value>  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn SetInstanceNetworkInfo(&mut self, InstanceId: UUID, PortMappings: Map<String, Value>) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        args.insert("PortMappings".to_string(), PortMappings.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/SetInstanceNetworkInfo".to_string(), args)
    }

    /// SetInstanceSuspended - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// * `param` Suspended bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn SetInstanceSuspended(&mut self, InstanceName: String, Suspended: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        args.insert("Suspended".to_string(), Suspended.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/SetInstanceSuspended".to_string(), args)
    }

    /// StartAllInstances - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn StartAllInstances(&mut self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/StartAllInstances".to_string(), args)
    }

    /// StartInstance - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn StartInstance(&mut self, InstanceName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/StartInstance".to_string(), args)
    }

    /// StopAllInstances - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn StopAllInstances(&mut self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/StopAllInstances".to_string(), args)
    }

    /// StopInstance - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn StopInstance(&mut self, InstanceName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/StopInstance".to_string(), args)
    }

    /// TestADSLoginDetails - 
    /// Name Description Optional
    /// * `param` url String  False
    /// * `param` username String  False
    /// * `param` password String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn TestADSLoginDetails(&mut self, url: String, username: String, password: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("url".to_string(), url.into());
        args.insert("username".to_string(), username.into());
        args.insert("password".to_string(), password.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/TestADSLoginDetails".to_string(), args)
    }

    /// UpdateDatastore - 
    /// Name Description Optional
    /// * `param` updatedDatastore Value  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpdateDatastore(&mut self, updatedDatastore: Value) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("updatedDatastore".to_string(), updatedDatastore.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/UpdateDatastore".to_string(), args)
    }

    /// UpdateDeploymentTemplate - 
    /// Name Description Optional
    /// * `param` templateToUpdate Value  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpdateDeploymentTemplate(&mut self, templateToUpdate: Value) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("templateToUpdate".to_string(), templateToUpdate.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/UpdateDeploymentTemplate".to_string(), args)
    }

    /// UpdateInstanceInfo - 
    /// Name Description Optional
    /// * `param` InstanceId String  False
    /// * `param` FriendlyName String  False
    /// * `param` Description String  False
    /// * `param` StartOnBoot bool  False
    /// * `param` Suspended bool  False
    /// * `param` ExcludeFromFirewall bool  False
    /// * `param` RunInContainer bool  False
    /// * `param` ContainerMemory i32  False
    /// * `param` MemoryPolicy Value  False
    /// * `param` ContainerMaxCPU Value  False
    /// * `param` ContainerImage String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpdateInstanceInfo(&mut self, InstanceId: String, FriendlyName: String, Description: String, StartOnBoot: bool, Suspended: bool, ExcludeFromFirewall: bool, RunInContainer: bool, ContainerMemory: i32, MemoryPolicy: Value, ContainerMaxCPU: Value, ContainerImage: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        args.insert("FriendlyName".to_string(), FriendlyName.into());
        args.insert("Description".to_string(), Description.into());
        args.insert("StartOnBoot".to_string(), StartOnBoot.into());
        args.insert("Suspended".to_string(), Suspended.into());
        args.insert("ExcludeFromFirewall".to_string(), ExcludeFromFirewall.into());
        args.insert("RunInContainer".to_string(), RunInContainer.into());
        args.insert("ContainerMemory".to_string(), ContainerMemory.into());
        args.insert("MemoryPolicy".to_string(), MemoryPolicy.into());
        args.insert("ContainerMaxCPU".to_string(), ContainerMaxCPU.into());
        args.insert("ContainerImage".to_string(), ContainerImage.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/UpdateInstanceInfo".to_string(), args)
    }

    /// UpdateTarget - 
    /// Name Description Optional
    /// * `param` TargetID UUID  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn UpdateTarget(&mut self, TargetID: UUID) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TargetID".to_string(), TargetID.into());
        self.ampapi.api_call::<Value>("ADSModule/UpdateTarget".to_string(), args)
    }

    /// UpdateTargetInfo - 
    /// Name Description Optional
    /// * `param` Id UUID  False
    /// * `param` FriendlyName String  False
    /// * `param` Url URL  False
    /// * `param` Description String  False
    /// * `param` Tags Vec<String>  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpdateTargetInfo(&mut self, Id: UUID, FriendlyName: String, Url: URL, Description: String, Tags: Vec<String>) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        args.insert("FriendlyName".to_string(), FriendlyName.into());
        args.insert("Url".to_string(), Url.into());
        args.insert("Description".to_string(), Description.into());
        args.insert("Tags".to_string(), Tags.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/UpdateTargetInfo".to_string(), args)
    }

    /// UpgradeAllInstances - 
    /// Name Description Optional
    /// * `param` RestartRunning bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpgradeAllInstances(&mut self, RestartRunning: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("RestartRunning".to_string(), RestartRunning.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/UpgradeAllInstances".to_string(), args)
    }

    /// UpgradeInstance - 
    /// Name Description Optional
    /// * `param` InstanceName String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpgradeInstance(&mut self, InstanceName: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceName".to_string(), InstanceName.into());
        self.ampapi.api_call::<ActionResult<Value>>("ADSModule/UpgradeInstance".to_string(), args)
    }

}
