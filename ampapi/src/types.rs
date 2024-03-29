#![allow(non_snake_case, dead_code)]

use serde::{ Deserialize, Serialize };

use serde_json::{Map, Value};

use serde_repr::{Serialize_repr, Deserialize_repr};

/// A Rust library for the AMP API
/// Author: p0t4t0sandwich

/// ActionResult - Generic response type for calls that return a result and a reason for failure
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult<T> {
    /// true if successful, false if not
    pub Status: bool,
    /// reason for failure
    pub Reason: String,
    /// result of the call
    pub Result: T,
}

/// AMPVersion - AMP version information
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AMPVersion {
    /// The major version number
    pub Major: i32,
    /// The minor version number
    pub Minor: i32,
    /// The build number
    pub Build: i32,
    /// The revision number
    pub Revision: i32,
    /// The major revision number
    pub MajorRevision: i32,
    /// The minor revision number
    pub MinorRevision: i32,
}

/// Branding - Defines the Branding object as part of the ModuleInfo object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branding {
    /// Whether to display branding
    pub DisplayBranding: bool,
    /// The page title
    pub PageTitle: String,
    /// The company name
    pub CompanyName: String,
    /// The welcome message
    pub WelcomeMessage: String,
    /// The branding message
    pub BrandingMessage: String,
    /// The short branding message
    pub ShortBrandingMessage: String,
    /// The URL
    pub URL: String,
    /// The support URL
    pub SupportURL: String,
    /// The support text
    pub SupportText: String,
    /// The submit ticket URL
    pub SubmitTicketURL: String,
    /// The logo URL
    pub LogoURL: String,
    /// The background URL
    pub BackgroundURL: String,
    /// The splash frame URL
    pub SplashFrameURL: String,
    /// The forgot password URL
    pub ForgotPasswordURL: String,
}

/// ConsoleEntry - Struct for the result of API.Core#GetUpdates.ConsoleEntries
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleEntry {
    /// The timestamp of the console entry
    pub Timestamp: String,
    /// The source of the console entry
    pub Source: String,
    /// The source ID of the console entry
    pub SourceId: String,
    /// The type of the console entry
    pub Type: String,
    /// The contents of the console entry
    pub Contents: String,
}

/// CPUInfo - CPU information object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUInfo {
    /// Number of CPU sockets
    pub Sockets: i32,
    /// Number of CPU cores
    pub Cores: i32,
    /// Number of CPU threads
    pub Threads: i32,
    /// CPU vendor
    pub Vendor: String,
    /// CPU model name
    pub ModelName: String,
    /// Total number of CPU cores
    pub TotalCores: i32,
    /// Total number of CPU threads
    pub TotalThreads: i32,
}

/// EndpointInfo - An application endpoint object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointInfo {
    /// The display name of the endpoint
    pub DisplayName: String,
    /// The endpoint address
    pub Endpoint: String,
    /// The URI of the endpoint
    pub Uri: String,
}

/// IADSInstance - An ADS instance object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IADSInstance {
    /// The ADS instance ID
    pub Id: i32,
    /// The instance ID
    pub InstanceId: UUID,
    /// The friendly name
    pub FriendlyName: String,
    /// The description
    pub Description: String,
    /// Whether the instance is disabled
    pub Disabled: bool,
    /// Whether the instance is remote
    pub IsRemote: bool,
    /// The platform information object
    pub Platform: PlatformInfo,
    /// The datastores
    pub Datastores: Vec<InstanceDatastore>,
    /// Whether the instance creates in containers
    pub CreatesInContainers: bool,
    /// The state
    pub State: State,
    /// The state reason
    pub StateReason: String,
    /// Whether the instance can create
    pub CanCreate: bool,
    /// The last updated
    pub LastUpdated: String,
    /// The available instances
    pub AvailableInstances: Vec<Instance>,
    /// The available IPs
    pub AvailableIPs: Vec<String>,
    /// The URL
    pub URL: String,
    /// The tags
    pub Tags: Vec<String>,
    /// The tag names
    pub TagNames: Vec<String>,
}

/// Instance - An instance object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    /// The instance ID
    pub InstanceID: UUID,
    /// The target ID
    pub TargetID: UUID,
    /// The instance name
    pub InstanceName: String,
    /// The friendly name
    pub FriendlyName: String,
    /// The description
    pub Description: String,
    /// The module
    pub Module: String,
    /// The module display name
    pub ModuleDisplayName: String,
    /// The AMP version
    pub AMPVersion: AMPVersion,
    /// Whether HTTPS is enabled
    pub IsHTTPS: bool,
    /// The IP address
    pub IP: String,
    /// The port
    pub Port: i32,
    /// Whether the instance is a daemon
    pub Daemon: bool,
    /// Whether the instance daemon autostarts
    pub DaemonAutostart: bool,
    /// Whether the instance is excluded from the firewall
    pub ExcludeFromFirewall: bool,
    /// Whether the instance is running
    pub Running: bool,
    /// The application state
    pub AppState: State,
    /// The tags
    pub Tags: Vec<String>,
    /// The disk usage in MB
    pub DiskUsageMB: i32,
    /// The release stream
    pub ReleaseStream: i32,
    /// The management mode
    pub ManagementMode: i32,
    /// Whether the instance is suspended#[allow(non_snake_case)]
    /// Whether the instance is a container instance
    pub IsContainerInstance: bool,
    /// The container memory in MB
    pub ContainerMemoryMB: i32,
    /// The container memory policy
    pub ContainerMemoryPolicy: i32,
    /// The container CPUs
    pub ContainerCPUs: f32,
    /// The specific Docker image
    pub SpecificDockerImage: String,
    /// The metrics
    pub Metrics: Map<String, Value>, // TODO: HashMap<String, Metric>
    /// The application endpoints
    pub ApplicationEndpoints: Vec<EndpointInfo>,
    /// The deployment arguments
    pub DeploymentArgs: Map<String, Value>, // TODO: HashMap<String, String>
    /// The display image source
    #[serde(default)]
    #[serde(with = "option_string_serde")]
    pub DisplayImageSource: Option<String>,
}

/// InstanceDatastore - A datastore object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceDatastore {
    /// The datastore ID
    pub Id: i32,
    /// The friendly name
    pub FriendlyName: String,
}

/// LicenceInfo - A struct to represent the object returned by the ADSModule#GetLicenceInfo() method
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenceInfo {
    /// The licence key
    pub LicenceKey: UUID,
    /// The grade
    pub Grade: UUID,
    /// The grade name
    pub GradeName: String,
    /// The product
    pub Product: UUID,
    /// The product name
    pub ProductName: String,
    /// The expiry date
    pub Expires: String,
    /// The usage
    pub Usage: i32,
}

/// LoginResult - Response type for API.Core.Login
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    /// Whether the login was successful
    pub success: bool,
    /// The user's permissions
    pub permissions: Vec<String>,
    /// The session ID
    pub sessionID: String,
    /// The remember me token
    pub rememberMeToken: String,
    /// The user information
    pub userInfo: UserInfo,
    /// The result reason
    pub resultReason: String,
    /// The result
    pub result: f64,
}

/// Message - Message type for API.Core.GetUpdates status messages (along with WS keep alive)
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The message ID
    pub Id: UUID,
    /// Whether the message has expired
    pub Expired: bool,
    /// The source of the message
    pub Source: String,
    /// The message
    pub Message: String,
    /// The age of the message in minutes
    pub AgeMinutes: i32,
}

/// option_string_serde - A serde module for optional strings
/// Author: p0t4t0sandwich
pub mod option_string_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    //noinspection RsUnnecessaryQualifications
    /// serialize - Serialize a string
    /// Author: p0t4t0sandwich
    /// * `string` - The string to serialize
    /// * `serializer` - The serializer
    /// Returns Result<String, serde::ser::Error>
    pub fn serialize<S>(string: &Option<String>, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match string {
            Some(string) => serializer.serialize_str(string),
            None => serializer.serialize_none(),
        }
    }

    //noinspection RsUnnecessaryQualifications
    /// deserialize - Deserialize a string
    /// Returns Result<Option<String>, serde::de::Error>
    pub fn deserialize<'de, D>(deserializer: D) -> core::result::Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: Option<String> = Option::deserialize(deserializer)?;
        Ok(string)
    }
}

/// Metric - A metric object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// The raw value
    pub RawValue: i32,
    /// The maximum value
    pub MaxValue: i32,
    /// The percentage
    pub Percent: f64,
    /// The units
    pub Units: String,
    /// The color
    #[serde(default)]
    #[serde(with = "option_string_serde")]
    pub Color: Option<String>,
    /// The second color
    #[serde(default)]
    #[serde(with = "option_string_serde")]
    pub Color2: Option<String>,
    /// The third color
    #[serde(default)]
    #[serde(with = "option_string_serde")]
    pub Color3: Option<String>,
}

/// ModuleInfo - A struct to represent the object returned by the ADSModule#GetModuleInfo() method
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// The name of the module
    pub Name: String,
    /// The author of the module
    pub Author: String,
    /// The application name
    pub AppName: String,
    /// Whether the module supports sleep mode
    pub SupportsSleep: bool,
    /// The loaded plugins
    pub LoadedPlugins: Vec<String>,
    /// The AMP version
    pub AMPVersion: AMPVersion,
    /// The AMP build
    pub AMPBuild: String,
    /// The tools version
    pub ToolsVersion: AMPVersion,
    /// The API version
    pub APIVersion: AMPVersion,
    /// The version codename
    pub VersionCodename: String,
    /// The timestamp
    pub Timestamp: String,
    /// The build spec
    pub BuildSpec: String,
    /// The branding object
    pub Branding: Branding,
    /// Whether analytics are enabled
    pub Analytics: bool,
    /// The feature set
    pub FeatureSet: Vec<String>,
    /// The instance ID
    pub InstanceId: UUID,
    /// The instance name
    pub InstanceName: String,
    /// The friendly name
    pub FriendlyName: String,
    /// The endpoint URI
    pub EndpointURI: String,
    /// The primary endpoint
    pub PrimaryEndpoint: String,
    /// The module name
    pub ModuleName: String,
    /// Whether the instance is remote
    pub IsRemoteInstance: bool,
    /// The display base URI
    pub DisplayBaseURI: String,
    /// Whether the module requires a full load
    pub RequiresFullLoad: bool,
    /// Whether remember me is allowed
    pub AllowRememberMe: bool,
}

/// PlatformInfo - Platform information object
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    /// The CPU information object
    pub CPUInfo: CPUInfo,
    /// The installed RAM in MB
    pub InstalledRAMMB: i32,
    /// The OS
    pub OS: i32,
    /// The platform name
    pub PlatformName: String,
    /// The system type
    pub SystemType: i32,
    /// The virtualization
    pub Virtualization: i32,
}

/// RemoteTargetInfo - A struct to represent the object returned by the ADSModule#GetTargetInfo() method
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteTargetInfo {
    /// The IP address list
    pub IPAddressList: Vec<String>,
    /// The platform information object
    pub PlatformInfo: PlatformInfo,
    /// The datastores
    pub Datastores: Vec<InstanceDatastore>,
    /// Whether the target deploys in containers
    pub DeploysInContainers: bool,
}

/// RunningTask - A running task object returned by the Core#GetTasks() method
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningTask {
    /// Whether the task is the primary task
    pub IsPrimaryTask: bool,
    /// The start time
    pub StartTime: String,
    /// The task ID
    pub Id: UUID,
    /// The task name
    pub Name: String,
    /// The task description
    pub Description: String,
    /// Whether the task is hidden from the UI
    pub HideFromUI: bool,
    /// Whether the task can be dismissed quickly
    pub FastDismiss: bool,
    /// The last update pushed
    pub LastUpdatePushed: String,
    /// The progress percentage
    pub ProgressPercent: f64,
    /// Whether the task is cancellable
    pub IsCancellable: bool,
    /// The origin
    pub Origin: String,
    /// Whether the task is indeterminate
    pub IsIndeterminate: bool,
    /// The state
    pub State: i32,
    /// The status
    pub Status: String,
}

/// SettingSpec - Response object for Core.GetSettingsSpec()
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingSpec {
    /// Whether the setting is read-only
    pub ReadOnly: bool,
    /// The name
    pub Name: String,
    /// The description
    pub Description: String,
    /// The category
    pub Category: String,
    /// The current value
    pub CurrentValue: String,
    /// The value type
    pub ValType: String,
    /// The enum values
    pub EnumValues: Vec<String>,
    /// Whether the enum values are deferred
    pub EnumValuesAreDeferred: bool,
    /// The node
    pub Node: String,
    /// The input type
    pub InputType: String,
    /// The selection source
    pub SelectionSource: String,
    /// Whether the setting is a provision spec
    pub IsProvisionSpec: bool,
    /// Whether the provision is read-only
    pub ReadOnlyProvision: bool,
    /// The actions
    pub Actions: Vec<String>,
    /// The keywords
    pub Keywords: Vec<String>,
    /// Whether the setting is always allowed to be read
    pub AlwaysAllowRead: bool,
    /// The tag
    pub Tag: String,
    /// The max length
    pub MaxLength: i32,
    /// The placeholder
    pub Placeholder: String,
    /// The suffix
    pub Suffix: String,
    /// The meta
    pub Meta: String,
    /// Whether the setting requires a restart
    pub RequiresRestart: bool,
}

/// State - Represents the state of an instance
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum State {
    Undefined = -1,
    Stopped = 0,
    PreStart = 5,
    /// The server is performing some first-time-start configuration.
    Configuring = 7,
    Starting = 10,
    Ready = 20,
    /// Server is in the middle of stopping, but once shutdown has finished it will automatically restart.
    Restarting = 30,
    Stopping = 40,
    PreparingForSleep = 45,
    /// The application should be able to be resumed quickly if using this state. Otherwise use Stopped.
    Sleeping = 50,
    /// The application is waiting for some external service/application to respond/become available.
    Waiting = 60,
    Installing = 70,
    Updating = 75,
    /// Used during installation, means that some user input is required to complete setup (authentication etc).
    AwaitingUserInput = 80,
    Failed = 100,
    Suspended = 200,
    Maintainence = 250,
    /// The state is unknown, or doesn't apply (for modules that don't start an external process)
    Indeterminate = 999,
}

/// Status - Struct for the result of API.Core.GetStatus
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    /// The state of the instance
    pub State: State,
    /// The uptime of the instance
    pub Uptime: String,
    /// The metrics
    pub Metrics: Map<String, Value>, // TODO: HashMap<String, Metric>
}

/// UpdateInfo - A struct to represent the object returned by the ADSModule#GetUpdateInfo() method
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// Whether an update is available
    pub UpdateAvailable: bool,
    /// The version of the update
    pub Version: String,
    /// The build of the update
    pub Build: String,
    /// The URL to the release notes
    pub ReleaseNotesURL: String,
    /// The version of the tools
    pub ToolsVersion: String,
    /// Whether the update is a patch
    pub PatchOnly: bool,
}

/// Updates - Response type for API.Core.GetUpdates
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Updates {
    /// The status of the server
    pub Status: Status,
    /// The console entries of the server
    pub ConsoleEntries: Vec<ConsoleEntry>,
    /// The messages of the server
    pub Messages: Vec<Message>,
    /// The tasks of the server
    pub Tasks: Vec<String>,
    /// The ports of the server
    pub Ports: Vec<String>,
}

/// UserInfo - Information about the user
/// Author: p0t4t0sandwich
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// The user ID
    pub ID: UUID,
    /// The username
    pub Username: String,
    /// The email address
    pub EmailAddress: String,
    /// Wether 2FA is enabled
    pub IsTwoFactorEnabled: bool,
    /// Whether the user is disabled
    pub Disabled: bool,
    /// The last login
    pub LastLogin: String,
    /// The Gravatar hash
    pub GravatarHash: String,
    /// Whether the user is an LDAP user
    pub IsLDAPUser: bool,
}

/// URL - A URL is a string that represents a URL
/// Author: p0t4t0sandwich
pub type URL = String;

/// UUID - UUID type
/// Author: p0t4t0sandwich
pub type UUID = String;
