use crate::{AMPAPI, types::LoginResult};

mod ads_module;
use ads_module::ADSModule;
use serde_json::Value;

mod core;
use core::Core;
use std::{collections::HashMap, io::Error};

mod email_sender_plugin;
use email_sender_plugin::EmailSenderPlugin;

mod file_manager_plugin;
use file_manager_plugin::FileManagerPlugin;

mod local_file_backup_plugin;
use local_file_backup_plugin::LocalFileBackupPlugin;

mod minecraft_module;
use minecraft_module::MinecraftModule;

mod rcon_plugin;
use rcon_plugin::RCONPlugin;

mod steamcmd_plugin;
use steamcmd_plugin::steamcmdplugin;


/// struct CommonAPI
/// Author: p0t4t0sandich
#[derive(Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct CommonAPI {
    ampapi: AMPAPI,
    pub Core: Core,
    pub EmailSenderPlugin: EmailSenderPlugin,
    pub FileManagerPlugin: FileManagerPlugin,
    pub LocalFileBackupPlugin: LocalFileBackupPlugin,
}

/// impl From<Module> for CommonAPI
/// Author: p0t4t0sandich
impl From<Module> for CommonAPI {
    fn from(module: Module) -> Self {
        match module {
            Module::CommonAPI(common_api) => common_api,
            _ => panic!("Cannot convert from {:?} to CommonAPI", module)
        }
    }
}

/// impl CommonAPI
/// Author: p0t4t0sandich
#[allow(non_snake_case, dead_code)]
impl CommonAPI {
    /// CommonAPI.new - Create a new CommonAPI struct
    /// Author: p0t4t0sandich
    /// * `base_uri` - The base URI of the AMP instance
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///   * Optional if using a remember me token
    /// * `remember_me_token` - The remember me token to use for authentication
    ///   * Optional
    /// * `session_id` - The session ID to use for authentication
    ///   * Optional, will be instantiated on login
    /// Returns CommonAPI
    pub fn new(base_uri: String, username: String, password: String, remember_me_token: String, session_id: String) -> CommonAPI {
        let ampapi = AMPAPI::new(base_uri, username, password, remember_me_token, session_id);

        let mut commonAPI = CommonAPI {
            ampapi: ampapi.clone(),
            Core: Core::new(ampapi.clone()),
            EmailSenderPlugin: EmailSenderPlugin::new(ampapi.clone()),
            FileManagerPlugin: FileManagerPlugin::new(ampapi.clone()),
            LocalFileBackupPlugin: LocalFileBackupPlugin::new(ampapi.clone()),
        };

        if !commonAPI.ampapi.username.is_empty()
            && (!commonAPI.ampapi.password.is_empty() || !commonAPI.ampapi.remember_me_token.is_empty())
        {
            let _ = commonAPI.login();
        }

        commonAPI
    }

    /// CommonAPI.login - Simplified login function
    /// Author: p0t4t0sandich
    /// Returns Result<LoginResult, reqwest::Error>
    pub fn login(&mut self) -> Result<crate::types::LoginResult, reqwest::Error> {
        let login_result = self.ampapi.login().unwrap();

        if login_result.success {
            self.ampapi.session_id = login_result.sessionID.clone();
            self.ampapi.remember_me_token = login_result.rememberMeToken.clone();

            // Update the session ID and remember me token of submodules
            self.Core.ampapi.session_id = login_result.sessionID.clone();
            self.Core.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.EmailSenderPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.EmailSenderPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.FileManagerPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.FileManagerPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.LocalFileBackupPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.LocalFileBackupPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
        }

        Ok(login_result)
    }    
}

/// struct GenericModule
/// Author: p0t4t0sandich
#[derive(Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct GenericModule {
    ampapi: AMPAPI,
    pub Core: Core,
    pub EmailSenderPlugin: EmailSenderPlugin,
    pub FileManagerPlugin: FileManagerPlugin,
    pub LocalFileBackupPlugin: LocalFileBackupPlugin,
    pub RCONPlugin: RCONPlugin,
    pub steamcmdplugin: steamcmdplugin,
}

/// impl From<Module> for GenericModule
/// Author: p0t4t0sandich
impl From<Module> for GenericModule {
    fn from(module: Module) -> Self {
        match module {
            Module::GenericModule(generic_module) => generic_module,
            _ => panic!("Cannot convert from {:?} to GenericModule", module)
        }
    }
}

/// impl GenericModule
/// Author: p0t4t0sandich
#[allow(non_snake_case, dead_code)]
impl GenericModule {
    /// GenericModule.new - Create a new GenericModule struct
    /// Author: p0t4t0sandich
    /// * `base_uri` - The base URI of the AMP instance
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///   * Optional if using a remember me token
    /// * `remember_me_token` - The remember me token to use for authentication
    ///   * Optional
    /// * `session_id` - The session ID to use for authentication
    ///   * Optional, will be instantiated on login
    /// Returns GenericModule
    pub fn new(base_uri: String, username: String, password: String, remember_me_token: String, session_id: String) -> GenericModule {
        let ampapi = AMPAPI::new(base_uri, username, password, remember_me_token, session_id);

        let mut genericModule = GenericModule {
            ampapi: ampapi.clone(),
            Core: Core::new(ampapi.clone()),
            EmailSenderPlugin: EmailSenderPlugin::new(ampapi.clone()),
            FileManagerPlugin: FileManagerPlugin::new(ampapi.clone()),
            LocalFileBackupPlugin: LocalFileBackupPlugin::new(ampapi.clone()),
            RCONPlugin: RCONPlugin::new(ampapi.clone()),
            steamcmdplugin: steamcmdplugin::new(ampapi.clone()),
        };

        if !genericModule.ampapi.username.is_empty()
            && (!genericModule.ampapi.password.is_empty() || !genericModule.ampapi.remember_me_token.is_empty())
        {
            let _ = genericModule.login();
        }

        genericModule
    }

    /// GenericModule.login - Simplified login function
    /// Author: p0t4t0sandich
    /// Returns Result<LoginResult, reqwest::Error>
    pub fn login(&mut self) -> Result<crate::types::LoginResult, reqwest::Error> {
        let login_result = self.ampapi.login().unwrap();

        if login_result.success {
            self.ampapi.session_id = login_result.sessionID.clone();
            self.ampapi.remember_me_token = login_result.rememberMeToken.clone();

            // Update the session ID and remember me token of submodules
            self.Core.ampapi.session_id = login_result.sessionID.clone();
            self.Core.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.EmailSenderPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.EmailSenderPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.FileManagerPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.FileManagerPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.LocalFileBackupPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.LocalFileBackupPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.RCONPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.RCONPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.steamcmdplugin.ampapi.session_id = login_result.sessionID.clone();
            self.steamcmdplugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
        }

        Ok(login_result)
    }
}

/// struct Minecraft
/// Author: p0t4t0sandich
#[derive(Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct Minecraft {
    ampapi: AMPAPI,
    pub Core: Core,
    pub EmailSenderPlugin: EmailSenderPlugin,
    pub FileManagerPlugin: FileManagerPlugin,
    pub LocalFileBackupPlugin: LocalFileBackupPlugin,
    pub MinecraftModule: MinecraftModule,
}

/// impl From<Module> for Minecraft
/// Author: p0t4t0sandich
impl From<Module> for Minecraft {
    fn from(module: Module) -> Self {
        match module {
            Module::Minecraft(minecraft) => minecraft,
            _ => panic!("Cannot convert from {:?} to Minecraft", module)
        }
    }
}

/// impl MinecraftModule
/// Author: p0t4t0sandich
#[allow(non_snake_case, dead_code)]
impl Minecraft {
    /// Minecraft.new - Create a new Minecraft struct
    /// Author: p0t4t0sandich
    /// * `base_uri` - The base URI of the AMP instance
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///   * Optional if using a remember me token
    /// * `remember_me_token` - The remember me token to use for authentication
    ///   * Optional
    /// * `session_id` - The session ID to use for authentication
    ///   * Optional, will be instantiated on login
    /// Returns Minecraft
    pub fn new(base_uri: String, username: String, password: String, remember_me_token: String, session_id: String) -> Minecraft {
        let ampapi = AMPAPI::new(base_uri, username, password, remember_me_token, session_id);

        let mut minecraft = Minecraft {
            ampapi: ampapi.clone(),
            Core: Core::new(ampapi.clone()),
            EmailSenderPlugin: EmailSenderPlugin::new(ampapi.clone()),
            FileManagerPlugin: FileManagerPlugin::new(ampapi.clone()),
            LocalFileBackupPlugin: LocalFileBackupPlugin::new(ampapi.clone()),
            MinecraftModule: MinecraftModule::new(ampapi.clone()),
        };

        if !minecraft.ampapi.username.is_empty()
            && (!minecraft.ampapi.password.is_empty() || !minecraft.ampapi.remember_me_token.is_empty())
        {
            let _ = minecraft.login();
        }

        minecraft
    }

    /// Minecraft.login - Simplified login function
    /// Author: p0t4t0sandich
    /// Returns Result<LoginResult, reqwest::Error>
    pub fn login(&mut self) -> Result<crate::types::LoginResult, reqwest::Error> {
        let login_result = self.ampapi.login().unwrap();

        if login_result.success {
            self.ampapi.session_id = login_result.sessionID.clone();
            self.ampapi.remember_me_token = login_result.rememberMeToken.clone();

            // Update the session ID and remember me token of submodules
            self.Core.ampapi.session_id = login_result.sessionID.clone();
            self.Core.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.EmailSenderPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.EmailSenderPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.FileManagerPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.FileManagerPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.LocalFileBackupPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.LocalFileBackupPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.MinecraftModule.ampapi.session_id = login_result.sessionID.clone();
            self.MinecraftModule.ampapi.remember_me_token = login_result.rememberMeToken.clone();
        }

        Ok(login_result)
    }
}

/// struct ADS
/// Author: p0t4t0sandich
#[derive(Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct ADS {
    ampapi: AMPAPI,
    pub Core: Core,
    pub EmailSenderPlugin: EmailSenderPlugin,
    pub FileManagerPlugin: FileManagerPlugin,
    pub LocalFileBackupPlugin: LocalFileBackupPlugin,
    pub ADSModule: ADSModule,
}

/// impl From<Module> for ADS
/// Author: p0t4t0sandich
impl From<Module> for ADS {
    fn from(module: Module) -> Self {
        match module {
            Module::ADS(ads) => ads,
            _ => panic!("Cannot convert from {:?} to ADS", module)
        }
    }
}

/// enum Module
/// Author: p0t4t0sandich
#[derive(Debug, Clone)]
pub enum Module {
    ADS(ADS),
    GenericModule(GenericModule),
    Minecraft(Minecraft),
    CommonAPI(CommonAPI),
}

/// impl ADS
/// Author: p0t4t0sandich
#[allow(non_snake_case, dead_code)]
impl ADS {
    /// ADS.new - Create a new ADS struct
    /// Author: p0t4t0sandich
    /// * `base_uri` - The base URI of the AMP instance
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///   * Optional if using a remember me token
    /// * `remember_me_token` - The remember me token to use for authentication
    ///   * Optional
    /// * `session_id` - The session ID to use for authentication
    ///   * Optional, will be instantiated on login
    /// Returns ADS
    pub fn new(base_uri: String, username: String, password: String, remember_me_token: String, session_id: String) -> ADS {
        let ampapi = AMPAPI::new(base_uri, username, password, remember_me_token, session_id);

        let mut ads = ADS {
            ampapi: ampapi.clone(),
            Core: Core::new(ampapi.clone()),
            EmailSenderPlugin: EmailSenderPlugin::new(ampapi.clone()),
            FileManagerPlugin: FileManagerPlugin::new(ampapi.clone()),
            LocalFileBackupPlugin: LocalFileBackupPlugin::new(ampapi.clone()),
            ADSModule: ADSModule::new(ampapi.clone()),
        };

        if !ads.ampapi.username.is_empty()
            && (!ads.ampapi.password.is_empty() || !ads.ampapi.remember_me_token.is_empty())
        {
            let _ = ads.login();
        }

        ads
    }

    /// ADS.login - Simplified login function
    /// Author: p0t4t0sandich
    /// Returns Result<LoginResult, reqwest::Error>
    pub fn login(&mut self) -> Result<crate::types::LoginResult, reqwest::Error> {
        let login_result = self.ampapi.login().unwrap();

        if login_result.success {
            self.ampapi.session_id = login_result.sessionID.clone();
            self.ampapi.remember_me_token = login_result.rememberMeToken.clone();

            // Update the session ID and remember me token of submodules
            self.Core.ampapi.session_id = login_result.sessionID.clone();
            self.Core.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.EmailSenderPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.EmailSenderPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.FileManagerPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.FileManagerPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.LocalFileBackupPlugin.ampapi.session_id = login_result.sessionID.clone();
            self.LocalFileBackupPlugin.ampapi.remember_me_token = login_result.rememberMeToken.clone();
            self.ADSModule.ampapi.session_id = login_result.sessionID.clone();
            self.ADSModule.ampapi.remember_me_token = login_result.rememberMeToken.clone();
        }

        Ok(login_result)
    }

    /// ADS.instance_login - Function to proxy a login to an instance
    /// Author: p0t4t0sandich
    /// * `instance_id` - The instance ID to login to
    /// * `module` - The type of module you're logging into
    /// Returns Result<T, reqwest::Error>
    pub fn instance_login(&self, instance_id: String, module: String) -> Result<Module, Error> {
        let mut args = HashMap::new();
        args.insert("username".to_string(), Value::String(self.ampapi.username.clone()));
        args.insert("password".to_string(), Value::String(self.ampapi.password.clone()));
        args.insert("token".to_string(), Value::String("".to_string()));
        args.insert("rememberMe".to_string(), Value::Bool(true));

        let loginResult: LoginResult = self.ampapi.api_call::<LoginResult>(format!("ADSModule/Servers/{}/API/Core/Login", instance_id), args).unwrap();

        if loginResult.success {
            // Prepare the parameters for the instance
            let new_base_uri = format!("{}API/ADSModule/Servers/{}", self.ampapi.base_uri, instance_id);
            let remember_me_token = loginResult.rememberMeToken.clone();
            let session_id = loginResult.sessionID.clone();

            // Return the correct module
            match module.as_str() {
                "ADS" => {
                    let mut ads = ADS::new(new_base_uri, self.ampapi.username.clone(), "".to_string(), remember_me_token, session_id);
                    ads.login().unwrap();
                    Ok(Module::ADS(ads))
                },
                "GenericModule" => {
                    let mut generic_module = GenericModule::new(new_base_uri, self.ampapi.username.clone(), "".to_string(), remember_me_token, session_id);
                    generic_module.login().unwrap();
                    Ok(Module::GenericModule(generic_module))
                },
                "Minecraft" => {
                    let mut minecraft = Minecraft::new(new_base_uri, self.ampapi.username.clone(), "".to_string(), remember_me_token, session_id);
                    minecraft.login().unwrap();
                    Ok(Module::Minecraft(minecraft))
                },
                _ => {
                    let mut common_api = CommonAPI::new(new_base_uri, self.ampapi.username.clone(), "".to_string(), remember_me_token, session_id);
                    common_api.login().unwrap();
                    Ok(Module::CommonAPI(common_api))
                }
            }
        } else {
            Err(Error::new(std::io::ErrorKind::Other, "Login failed"))
        }
    }
}
