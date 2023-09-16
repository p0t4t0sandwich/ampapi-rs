use crate::{AMPAPI, types::*};

use std::collections::HashMap;

#[allow(unused_imports)]
use serde_json::{Value, Map};

/// A Rust library for the AMP API
/// Author: p0t4t0sandich

/// struct Core
#[derive(Debug, Clone)]
pub struct Core {
    pub ampapi: AMPAPI
}

#[allow(non_snake_case, dead_code, unused_mut)]
impl Core {
	///new - Creates a new Core object
	pub fn new(ampapi: AMPAPI) -> Core {
		Core {
			ampapi
		}
	}

    /// AcknowledgeAMPUpdate - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn AcknowledgeAMPUpdate(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/AcknowledgeAMPUpdate".to_string(), args)
    }

    /// AddEventTrigger - 
    /// Name Description Optional
    /// * `param` triggerId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn AddEventTrigger(&self, triggerId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("triggerId".to_string(), triggerId.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/AddEventTrigger".to_string(), args)
    }

    /// AddIntervalTrigger - 
    /// Name Description Optional
    /// * `param` months Vec<i32>  False
    /// * `param` days Vec<i32>  False
    /// * `param` hours Vec<i32>  False
    /// * `param` minutes Vec<i32>  False
    /// * `param` daysOfMonth Vec<i32>  False
    /// * `param` description String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn AddIntervalTrigger(&self, months: Vec<i32>, days: Vec<i32>, hours: Vec<i32>, minutes: Vec<i32>, daysOfMonth: Vec<i32>, description: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("months".to_string(), months.into());
        args.insert("days".to_string(), days.into());
        args.insert("hours".to_string(), hours.into());
        args.insert("minutes".to_string(), minutes.into());
        args.insert("daysOfMonth".to_string(), daysOfMonth.into());
        args.insert("description".to_string(), description.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/AddIntervalTrigger".to_string(), args)
    }

    /// AddTask - 
    /// Name Description Optional
    /// * `param` TriggerID UUID  False
    /// * `param` MethodID String  False
    /// * `param` ParameterMapping Map<String, Value>  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn AddTask(&self, TriggerID: UUID, MethodID: String, ParameterMapping: Map<String, Value>) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TriggerID".to_string(), TriggerID.into());
        args.insert("MethodID".to_string(), MethodID.into());
        args.insert("ParameterMapping".to_string(), ParameterMapping.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/AddTask".to_string(), args)
    }

    /// AsyncTest - DEV: Async test method
    /// Name Description Optional
    /// Return core::result::Result<Task<String>, reqwest::Error>
    pub fn AsyncTest(&self, ) -> core::result::Result<Task<String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Task<String>>("Core/AsyncTest".to_string(), args)
    }

    /// CancelTask - 
    /// Name Description Optional
    /// * `param` TaskId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn CancelTask(&self, TaskId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TaskId".to_string(), TaskId.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/CancelTask".to_string(), args)
    }

    /// ChangeTaskOrder - 
    /// Name Description Optional
    /// * `param` TriggerID UUID  False
    /// * `param` TaskID UUID  False
    /// * `param` NewOrder i32  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn ChangeTaskOrder(&self, TriggerID: UUID, TaskID: UUID, NewOrder: i32) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TriggerID".to_string(), TriggerID.into());
        args.insert("TaskID".to_string(), TaskID.into());
        args.insert("NewOrder".to_string(), NewOrder.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/ChangeTaskOrder".to_string(), args)
    }

    /// ChangeUserPassword - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// * `param` OldPassword String  False
    /// * `param` NewPassword String  False
    /// * `param` TwoFactorPIN String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn ChangeUserPassword(&self, Username: String, OldPassword: String, NewPassword: String, TwoFactorPIN: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        args.insert("OldPassword".to_string(), OldPassword.into());
        args.insert("NewPassword".to_string(), NewPassword.into());
        args.insert("TwoFactorPIN".to_string(), TwoFactorPIN.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/ChangeUserPassword".to_string(), args)
    }

    /// ConfirmTwoFactorSetup - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// * `param` TwoFactorCode String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn ConfirmTwoFactorSetup(&self, Username: String, TwoFactorCode: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        args.insert("TwoFactorCode".to_string(), TwoFactorCode.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/ConfirmTwoFactorSetup".to_string(), args)
    }

    /// CreateRole - 
    /// Name Description Optional
    /// * `param` Name String  False
    /// * `param` AsCommonRole bool  True
    /// Return core::result::Result<Task<ActionResult<UUID>>, reqwest::Error>
    pub fn CreateRole(&self, Name: String, AsCommonRole: bool) -> core::result::Result<Task<ActionResult<UUID>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Name".to_string(), Name.into());
        args.insert("AsCommonRole".to_string(), AsCommonRole.into());
        self.ampapi.api_call::<Task<ActionResult<UUID>>>("Core/CreateRole".to_string(), args)
    }

    /// CreateTestTask - DEV: Creates a non-ending task with 50% progress for testing purposes
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn CreateTestTask(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/CreateTestTask".to_string(), args)
    }

    /// CreateUser - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// Return core::result::Result<Task<ActionResult<UUID>>, reqwest::Error>
    pub fn CreateUser(&self, Username: String) -> core::result::Result<Task<ActionResult<UUID>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        self.ampapi.api_call::<Task<ActionResult<UUID>>>("Core/CreateUser".to_string(), args)
    }

    /// CurrentSessionHasPermission - 
    /// Name Description Optional
    /// * `param` PermissionNode String  False
    /// Return core::result::Result<bool, reqwest::Error>
    pub fn CurrentSessionHasPermission(&self, PermissionNode: String) -> core::result::Result<bool, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("PermissionNode".to_string(), PermissionNode.into());
        self.ampapi.api_call::<bool>("Core/CurrentSessionHasPermission".to_string(), args)
    }

    /// DeleteInstanceUsers - 
    /// Name Description Optional
    /// * `param` InstanceId UUID  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn DeleteInstanceUsers(&self, InstanceId: UUID) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("InstanceId".to_string(), InstanceId.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/DeleteInstanceUsers".to_string(), args)
    }

    /// DeleteRole - 
    /// Name Description Optional
    /// * `param` RoleId UUID  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn DeleteRole(&self, RoleId: UUID) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("RoleId".to_string(), RoleId.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/DeleteRole".to_string(), args)
    }

    /// DeleteTask - 
    /// Name Description Optional
    /// * `param` TriggerID UUID  False
    /// * `param` TaskID UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DeleteTask(&self, TriggerID: UUID, TaskID: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TriggerID".to_string(), TriggerID.into());
        args.insert("TaskID".to_string(), TaskID.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/DeleteTask".to_string(), args)
    }

    /// DeleteTrigger - 
    /// Name Description Optional
    /// * `param` TriggerID UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DeleteTrigger(&self, TriggerID: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TriggerID".to_string(), TriggerID.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/DeleteTrigger".to_string(), args)
    }

    /// DeleteUser - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn DeleteUser(&self, Username: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/DeleteUser".to_string(), args)
    }

    /// DisableTwoFactor - 
    /// Name Description Optional
    /// * `param` Password String  False
    /// * `param` TwoFactorCode String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn DisableTwoFactor(&self, Password: String, TwoFactorCode: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Password".to_string(), Password.into());
        args.insert("TwoFactorCode".to_string(), TwoFactorCode.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/DisableTwoFactor".to_string(), args)
    }

    /// DismissAllTasks - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DismissAllTasks(&self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("Core/DismissAllTasks".to_string(), args)
    }

    /// DismissTask - 
    /// Name Description Optional
    /// * `param` TaskId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn DismissTask(&self, TaskId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TaskId".to_string(), TaskId.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/DismissTask".to_string(), args)
    }

    /// EditIntervalTrigger - 
    /// Name Description Optional
    /// * `param` Id UUID  False
    /// * `param` months Vec<i32>  False
    /// * `param` days Vec<i32>  False
    /// * `param` hours Vec<i32>  False
    /// * `param` minutes Vec<i32>  False
    /// * `param` daysOfMonth Vec<i32>  False
    /// * `param` description String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn EditIntervalTrigger(&self, Id: UUID, months: Vec<i32>, days: Vec<i32>, hours: Vec<i32>, minutes: Vec<i32>, daysOfMonth: Vec<i32>, description: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        args.insert("months".to_string(), months.into());
        args.insert("days".to_string(), days.into());
        args.insert("hours".to_string(), hours.into());
        args.insert("minutes".to_string(), minutes.into());
        args.insert("daysOfMonth".to_string(), daysOfMonth.into());
        args.insert("description".to_string(), description.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/EditIntervalTrigger".to_string(), args)
    }

    /// EditTask - 
    /// Name Description Optional
    /// * `param` TriggerID UUID  False
    /// * `param` TaskID UUID  False
    /// * `param` ParameterMapping Map<String, Value>  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn EditTask(&self, TriggerID: UUID, TaskID: UUID, ParameterMapping: Map<String, Value>) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("TriggerID".to_string(), TriggerID.into());
        args.insert("TaskID".to_string(), TaskID.into());
        args.insert("ParameterMapping".to_string(), ParameterMapping.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/EditTask".to_string(), args)
    }

    /// EnableTwoFactor - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// * `param` Password String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn EnableTwoFactor(&self, Username: String, Password: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        args.insert("Password".to_string(), Password.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/EnableTwoFactor".to_string(), args)
    }

    /// EndUserSession - 
    /// Name Description Optional
    /// * `param` Id UUID  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn EndUserSession(&self, Id: UUID) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        self.ampapi.api_call::<Value>("Core/EndUserSession".to_string(), args)
    }

    /// GetAMPRolePermissions - 
    /// Name Description Optional
    /// * `param` RoleId UUID  False
    /// Return core::result::Result<Task<Vec<String>>, reqwest::Error>
    pub fn GetAMPRolePermissions(&self, RoleId: UUID) -> core::result::Result<Task<Vec<String>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("RoleId".to_string(), RoleId.into());
        self.ampapi.api_call::<Task<Vec<String>>>("Core/GetAMPRolePermissions".to_string(), args)
    }

    /// GetAMPUserInfo - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// Return core::result::Result<Task<UserInfo>, reqwest::Error>
    pub fn GetAMPUserInfo(&self, Username: String) -> core::result::Result<Task<UserInfo>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        self.ampapi.api_call::<Task<UserInfo>>("Core/GetAMPUserInfo".to_string(), args)
    }

    /// GetAMPUsersSummary - 
    /// Name Description Optional
    /// Return core::result::Result<Task<Vec<Value>>, reqwest::Error>
    pub fn GetAMPUsersSummary(&self, ) -> core::result::Result<Task<Vec<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Task<Vec<Value>>>("Core/GetAMPUsersSummary".to_string(), args)
    }

    /// GetAPISpec - 
    /// Name Description Optional
    /// Return core::result::Result<HashMap<String, HashMap<String, Value>>, reqwest::Error>
    pub fn GetAPISpec(&self, ) -> core::result::Result<HashMap<String, HashMap<String, Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<HashMap<String, HashMap<String, Value>>>("Core/GetAPISpec".to_string(), args)
    }

    /// GetActiveAMPSessions - 
    /// Name Description Optional
    /// Return core::result::Result<Result<Vec<Value>>, reqwest::Error>
    pub fn GetActiveAMPSessions(&self, ) -> core::result::Result<Result<Vec<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<Vec<Value>>>("Core/GetActiveAMPSessions".to_string(), args)
    }

    /// GetAllAMPUserInfo - 
    /// Name Description Optional
    /// Return core::result::Result<Task<Vec<UserInfo>>, reqwest::Error>
    pub fn GetAllAMPUserInfo(&self, ) -> core::result::Result<Task<Vec<UserInfo>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Task<Vec<UserInfo>>>("Core/GetAllAMPUserInfo".to_string(), args)
    }

    /// GetAuditLogEntries - 
    /// Name Description Optional
    /// * `param` Before Option<Value>  False
    /// * `param` Count i32  False
    /// Return core::result::Result<Result<Vec<Value>>, reqwest::Error>
    pub fn GetAuditLogEntries(&self, Before: Option<Value>, Count: i32) -> core::result::Result<Result<Vec<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Before".to_string(), Before.into());
        args.insert("Count".to_string(), Count.into());
        self.ampapi.api_call::<Result<Vec<Value>>>("Core/GetAuditLogEntries".to_string(), args)
    }

    /// GetConfig - 
    /// Name Description Optional
    /// * `param` node String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetConfig(&self, node: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("node".to_string(), node.into());
        self.ampapi.api_call::<Value>("Core/GetConfig".to_string(), args)
    }

    /// GetConfigs - 
    /// Name Description Optional
    /// * `param` nodes Vec<String>  False
    /// Return core::result::Result<Result<HashMap<String, Value>>, reqwest::Error>
    pub fn GetConfigs(&self, nodes: Vec<String>) -> core::result::Result<Result<HashMap<String, Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("nodes".to_string(), nodes.into());
        self.ampapi.api_call::<Result<HashMap<String, Value>>>("Core/GetConfigs".to_string(), args)
    }

    /// GetDiagnosticsInfo - 
    /// Name Description Optional
    /// Return core::result::Result<Map<String, Value>, reqwest::Error>
    pub fn GetDiagnosticsInfo(&self, ) -> core::result::Result<Map<String, Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Map<String, Value>>("Core/GetDiagnosticsInfo".to_string(), args)
    }

    /// GetModuleInfo - 
    /// Name Description Optional
    /// Return core::result::Result<Result<ModuleInfo>, reqwest::Error>
    pub fn GetModuleInfo(&self, ) -> core::result::Result<Result<ModuleInfo>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<ModuleInfo>>("Core/GetModuleInfo".to_string(), args)
    }

    /// GetNewGuid - 
    /// Name Description Optional
    /// Return core::result::Result<UUID, reqwest::Error>
    pub fn GetNewGuid(&self, ) -> core::result::Result<UUID, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<UUID>("Core/GetNewGuid".to_string(), args)
    }

    /// GetPermissionsSpec - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetPermissionsSpec(&self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("Core/GetPermissionsSpec".to_string(), args)
    }

    /// GetPortSummaries - 
    /// Name Description Optional
    /// Return core::result::Result<Result<Vec<Value>>, reqwest::Error>
    pub fn GetPortSummaries(&self, ) -> core::result::Result<Result<Vec<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<Vec<Value>>>("Core/GetPortSummaries".to_string(), args)
    }

    /// GetProvisionSpec - 
    /// Name Description Optional
    /// Return core::result::Result<Vec<Value>, reqwest::Error>
    pub fn GetProvisionSpec(&self, ) -> core::result::Result<Vec<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Vec<Value>>("Core/GetProvisionSpec".to_string(), args)
    }

    /// GetRemoteLoginToken - 
    /// Name Description Optional
    /// * `param` Description String  True
    /// * `param` IsTemporary bool  True
    /// Return core::result::Result<Task<String>, reqwest::Error>
    pub fn GetRemoteLoginToken(&self, Description: String, IsTemporary: bool) -> core::result::Result<Task<String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Description".to_string(), Description.into());
        args.insert("IsTemporary".to_string(), IsTemporary.into());
        self.ampapi.api_call::<Task<String>>("Core/GetRemoteLoginToken".to_string(), args)
    }

    /// GetRole - 
    /// Name Description Optional
    /// * `param` RoleId UUID  False
    /// Return core::result::Result<Task<Value>, reqwest::Error>
    pub fn GetRole(&self, RoleId: UUID) -> core::result::Result<Task<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("RoleId".to_string(), RoleId.into());
        self.ampapi.api_call::<Task<Value>>("Core/GetRole".to_string(), args)
    }

    /// GetRoleData - 
    /// Name Description Optional
    /// Return core::result::Result<Task<Vec<Value>>, reqwest::Error>
    pub fn GetRoleData(&self, ) -> core::result::Result<Task<Vec<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Task<Vec<Value>>>("Core/GetRoleData".to_string(), args)
    }

    /// GetRoleIds - 
    /// Name Description Optional
    /// Return core::result::Result<Task<HashMap<UUID, Value>>, reqwest::Error>
    pub fn GetRoleIds(&self, ) -> core::result::Result<Task<HashMap<UUID, Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Task<HashMap<UUID, Value>>>("Core/GetRoleIds".to_string(), args)
    }

    /// GetScheduleData - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetScheduleData(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/GetScheduleData".to_string(), args)
    }

    /// GetSettingValues - 
    /// Name Description Optional
    /// * `param` SettingNode String  False
    /// * `param` WithRefresh bool  True
    /// Return core::result::Result<HashMap<String, String>, reqwest::Error>
    pub fn GetSettingValues(&self, SettingNode: String, WithRefresh: bool) -> core::result::Result<HashMap<String, String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("SettingNode".to_string(), SettingNode.into());
        args.insert("WithRefresh".to_string(), WithRefresh.into());
        self.ampapi.api_call::<HashMap<String, String>>("Core/GetSettingValues".to_string(), args)
    }

    /// GetSettingsSpec - 
    /// Name Description Optional
    /// Return core::result::Result<SettingsSpec, reqwest::Error>
    pub fn GetSettingsSpec(&self, ) -> core::result::Result<SettingsSpec, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<SettingsSpec>("Core/GetSettingsSpec".to_string(), args)
    }

    /// GetStatus - 
    /// Name Description Optional
    /// Return core::result::Result<Status, reqwest::Error>
    pub fn GetStatus(&self, ) -> core::result::Result<Status, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Status>("Core/GetStatus".to_string(), args)
    }

    /// GetTasks - 
    /// Name Description Optional
    /// Return core::result::Result<Result<Vec<RunningTask>>, reqwest::Error>
    pub fn GetTasks(&self, ) -> core::result::Result<Result<Vec<RunningTask>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<Vec<RunningTask>>>("Core/GetTasks".to_string(), args)
    }

    /// GetTimeIntervalTrigger - 
    /// Name Description Optional
    /// * `param` Id UUID  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetTimeIntervalTrigger(&self, Id: UUID) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        self.ampapi.api_call::<Value>("Core/GetTimeIntervalTrigger".to_string(), args)
    }

    /// GetUpdateInfo - 
    /// Name Description Optional
    /// Return core::result::Result<Result<UpdateInfo>, reqwest::Error>
    pub fn GetUpdateInfo(&self, ) -> core::result::Result<Result<UpdateInfo>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<UpdateInfo>>("Core/GetUpdateInfo".to_string(), args)
    }

    /// GetUpdates - Gets changes to the server status, in addition to any notifications or console output that have occured since the last time GetUpdates() was called by the current session.
    /// Name Description Optional
    /// Return core::result::Result<Updates, reqwest::Error>
    pub fn GetUpdates(&self, ) -> core::result::Result<Updates, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Updates>("Core/GetUpdates".to_string(), args)
    }

    /// GetUserActionsSpec - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetUserActionsSpec(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/GetUserActionsSpec".to_string(), args)
    }

    /// GetUserInfo - 
    /// Name Description Optional
    /// * `param` UID String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetUserInfo(&self, UID: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("UID".to_string(), UID.into());
        self.ampapi.api_call::<Value>("Core/GetUserInfo".to_string(), args)
    }

    /// GetUserList - Returns a list of in-application users
    /// Name Description Optional
    /// Return core::result::Result<Result<HashMap<String, String>>, reqwest::Error>
    pub fn GetUserList(&self, ) -> core::result::Result<Result<HashMap<String, String>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<HashMap<String, String>>>("Core/GetUserList".to_string(), args)
    }

    /// GetWebauthnChallenge - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<String>, reqwest::Error>
    pub fn GetWebauthnChallenge(&self, ) -> core::result::Result<ActionResult<String>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<String>>("Core/GetWebauthnChallenge".to_string(), args)
    }

    /// GetWebauthnCredentialIDs - 
    /// Name Description Optional
    /// * `param` username String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetWebauthnCredentialIDs(&self, username: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("username".to_string(), username.into());
        self.ampapi.api_call::<Value>("Core/GetWebauthnCredentialIDs".to_string(), args)
    }

    /// GetWebauthnCredentialSummaries - 
    /// Name Description Optional
    /// Return core::result::Result<Result<Vec<Value>>, reqwest::Error>
    pub fn GetWebauthnCredentialSummaries(&self, ) -> core::result::Result<Result<Vec<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Result<Vec<Value>>>("Core/GetWebauthnCredentialSummaries".to_string(), args)
    }

    /// GetWebserverMetrics - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn GetWebserverMetrics(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/GetWebserverMetrics".to_string(), args)
    }

    /// Kill - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Kill(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/Kill".to_string(), args)
    }

    /// Login - 
    /// Name Description Optional
    /// * `param` username String  False
    /// * `param` password String  False
    /// * `param` token String  False
    /// * `param` rememberMe bool  False
    /// Return core::result::Result<LoginResult, reqwest::Error>
    pub fn Login(&self, username: String, password: String, token: String, rememberMe: bool) -> core::result::Result<LoginResult, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("username".to_string(), username.into());
        args.insert("password".to_string(), password.into());
        args.insert("token".to_string(), token.into());
        args.insert("rememberMe".to_string(), rememberMe.into());
        self.ampapi.api_call::<LoginResult>("Core/Login".to_string(), args)
    }

    /// Logout - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Logout(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/Logout".to_string(), args)
    }

    /// RefreshSettingValueList - 
    /// Name Description Optional
    /// * `param` Node String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RefreshSettingValueList(&self, Node: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Node".to_string(), Node.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/RefreshSettingValueList".to_string(), args)
    }

    /// RefreshSettingsSourceCache - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn RefreshSettingsSourceCache(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/RefreshSettingsSourceCache".to_string(), args)
    }

    /// RenameRole - 
    /// Name Description Optional
    /// * `param` RoleId UUID  False
    /// * `param` NewName String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn RenameRole(&self, RoleId: UUID, NewName: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("RoleId".to_string(), RoleId.into());
        args.insert("NewName".to_string(), NewName.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/RenameRole".to_string(), args)
    }

    /// ResetUserPassword - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// * `param` NewPassword String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn ResetUserPassword(&self, Username: String, NewPassword: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        args.insert("NewPassword".to_string(), NewPassword.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/ResetUserPassword".to_string(), args)
    }

    /// Restart - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn Restart(&self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("Core/Restart".to_string(), args)
    }

    /// RestartAMP - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn RestartAMP(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/RestartAMP".to_string(), args)
    }

    /// Resume - Allows the service to be re-started after previously being suspended.
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Resume(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/Resume".to_string(), args)
    }

    /// RevokeWebauthnCredential - 
    /// Name Description Optional
    /// * `param` ID i32  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RevokeWebauthnCredential(&self, ID: i32) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("ID".to_string(), ID.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/RevokeWebauthnCredential".to_string(), args)
    }

    /// RunEventTriggerImmediately - 
    /// Name Description Optional
    /// * `param` triggerId UUID  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn RunEventTriggerImmediately(&self, triggerId: UUID) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("triggerId".to_string(), triggerId.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/RunEventTriggerImmediately".to_string(), args)
    }

    /// SendConsoleMessage - 
    /// Name Description Optional
    /// * `param` message String  False
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn SendConsoleMessage(&self, message: String) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("message".to_string(), message.into());
        self.ampapi.api_call::<Value>("Core/SendConsoleMessage".to_string(), args)
    }

    /// SetAMPRolePermission - 
    /// Name Description Optional
    /// * `param` RoleId UUID  False
    /// * `param` PermissionNode String  False
    /// * `param` Enabled Option<bool>  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn SetAMPRolePermission(&self, RoleId: UUID, PermissionNode: String, Enabled: Option<bool>) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("RoleId".to_string(), RoleId.into());
        args.insert("PermissionNode".to_string(), PermissionNode.into());
        args.insert("Enabled".to_string(), Enabled.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/SetAMPRolePermission".to_string(), args)
    }

    /// SetAMPUserRoleMembership - 
    /// Name Description Optional
    /// * `param` UserId UUID  False
    /// * `param` RoleId UUID  False
    /// * `param` IsMember bool  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn SetAMPUserRoleMembership(&self, UserId: UUID, RoleId: UUID, IsMember: bool) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("UserId".to_string(), UserId.into());
        args.insert("RoleId".to_string(), RoleId.into());
        args.insert("IsMember".to_string(), IsMember.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/SetAMPUserRoleMembership".to_string(), args)
    }

    /// SetConfig - 
    /// Name Description Optional
    /// * `param` node String  False
    /// * `param` value String  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn SetConfig(&self, node: String, value: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("node".to_string(), node.into());
        args.insert("value".to_string(), value.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/SetConfig".to_string(), args)
    }

    /// SetConfigs - 
    /// Name Description Optional
    /// * `param` data Map<String, Value>  False
    /// Return core::result::Result<bool, reqwest::Error>
    pub fn SetConfigs(&self, data: Map<String, Value>) -> core::result::Result<bool, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("data".to_string(), data.into());
        self.ampapi.api_call::<bool>("Core/SetConfigs".to_string(), args)
    }

    /// SetTriggerEnabled - 
    /// Name Description Optional
    /// * `param` Id UUID  False
    /// * `param` Enabled bool  False
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn SetTriggerEnabled(&self, Id: UUID, Enabled: bool) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Id".to_string(), Id.into());
        args.insert("Enabled".to_string(), Enabled.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/SetTriggerEnabled".to_string(), args)
    }

    /// Sleep - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn Sleep(&self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("Core/Sleep".to_string(), args)
    }

    /// Start - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn Start(&self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("Core/Start".to_string(), args)
    }

    /// Stop - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Stop(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/Stop".to_string(), args)
    }

    /// Suspend - Prevents the current instance from being started, and stops it if it's currently running.
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn Suspend(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/Suspend".to_string(), args)
    }

    /// UpdateAMPInstance - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn UpdateAMPInstance(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/UpdateAMPInstance".to_string(), args)
    }

    /// UpdateAccountInfo - 
    /// Name Description Optional
    /// * `param` EmailAddress String  False
    /// * `param` TwoFactorPIN String  False
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn UpdateAccountInfo(&self, EmailAddress: String, TwoFactorPIN: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("EmailAddress".to_string(), EmailAddress.into());
        args.insert("TwoFactorPIN".to_string(), TwoFactorPIN.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/UpdateAccountInfo".to_string(), args)
    }

    /// UpdateApplication - 
    /// Name Description Optional
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn UpdateApplication(&self, ) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<ActionResult<Value>>("Core/UpdateApplication".to_string(), args)
    }

    /// UpdateUserInfo - 
    /// Name Description Optional
    /// * `param` Username String  False
    /// * `param` Disabled bool  False
    /// * `param` PasswordExpires bool  False
    /// * `param` CannotChangePassword bool  False
    /// * `param` MustChangePassword bool  False
    /// * `param` EmailAddress String  True
    /// Return core::result::Result<Task<ActionResult<Value>>, reqwest::Error>
    pub fn UpdateUserInfo(&self, Username: String, Disabled: bool, PasswordExpires: bool, CannotChangePassword: bool, MustChangePassword: bool, EmailAddress: String) -> core::result::Result<Task<ActionResult<Value>>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("Username".to_string(), Username.into());
        args.insert("Disabled".to_string(), Disabled.into());
        args.insert("PasswordExpires".to_string(), PasswordExpires.into());
        args.insert("CannotChangePassword".to_string(), CannotChangePassword.into());
        args.insert("MustChangePassword".to_string(), MustChangePassword.into());
        args.insert("EmailAddress".to_string(), EmailAddress.into());
        self.ampapi.api_call::<Task<ActionResult<Value>>>("Core/UpdateUserInfo".to_string(), args)
    }

    /// UpgradeAMP - 
    /// Name Description Optional
    /// Return core::result::Result<Value, reqwest::Error>
    pub fn UpgradeAMP(&self, ) -> core::result::Result<Value, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        self.ampapi.api_call::<Value>("Core/UpgradeAMP".to_string(), args)
    }

    /// WebauthnRegister - 
    /// Name Description Optional
    /// * `param` attestationObject String  False
    /// * `param` clientDataJSON String  False
    /// * `param` description String  True
    /// Return core::result::Result<ActionResult<Value>, reqwest::Error>
    pub fn WebauthnRegister(&self, attestationObject: String, clientDataJSON: String, description: String) -> core::result::Result<ActionResult<Value>, reqwest::Error> {
        let mut args: HashMap<String, Value> = HashMap::new();
        args.insert("attestationObject".to_string(), attestationObject.into());
        args.insert("clientDataJSON".to_string(), clientDataJSON.into());
        args.insert("description".to_string(), description.into());
        self.ampapi.api_call::<ActionResult<Value>>("Core/WebauthnRegister".to_string(), args)
    }

}
