#!/bin/python3
from __future__ import annotations

import sys

import requests
import json


type_dict = {
    "InstanceDatastore": "Value",
    "ActionResult": "ActionResult<Value>",
    "Int32": "i32",
    "IEnumerable<InstanceDatastore>": "Result<Vec<InstanceDatastore>>",
    "RunningTask": "Result<RunningTask>",
    "Task<RunningTask>": "Task<RunningTask>",
    "IEnumerable<JObject>": "Result<HashMap<String, Value>>",
    "Guid": "UUID",
    "IEnumerable<DeploymentTemplate>": "Result<Vec<Value>>",
    "String": "String",
    "DeploymentTemplate": "Value",
    "Boolean": "bool",
    "List<String>": "Vec<String>",
    "PostCreateActions": "Value",
    "Dictionary<String, String>": "Map<String, Value>", 
    "RemoteTargetInfo": "RemoteTargetInfo",
    "IEnumerable<ApplicationSpec>": "Result<Vec<Value>>",
    "Void": "Value",
    "IEnumerable<EndpointInfo>": "Result<Vec<EndpointInfo>>",
    "IEnumerable<IADSInstance>": "Result<Vec<IADSInstance>>",
    "JObject": "Value",
    "PortProtocol": "Value",
    "ActionResult<String>": "ActionResult<String>",
    "IADSInstance": "Result<IADSInstance>",
    "Uri": "URL",
    "IEnumerable<PortUsage>": "Result<Vec<Value>>",
    "Dictionary<String, Int32>": "Map<String, Value>",
    "LocalAMPInstance": "Value",
    "ContainerMemoryPolicy": "Value",
    "Single": "Value",
    "Int64": "i64",
    "FileChunkData": "Value",
    "IEnumerable<BackupManifest>": "Result<Vec<Value>>",
    "Nullable<DateTime>": "Option<Value>", # Optional?
    "IEnumerable<IAuditLogEntry>": "Result<Vec<Value>>",
    "Dictionary<String, IEnumerable<JObject>>": "HashMap<String, Vec<Value>>",
    "IDictionary<String, String>": "HashMap<String, String>",
    "List<JObject>": "Vec<Value>",
    "String[]": "Vec<String>",
    "Nullable<Boolean>": "Option<bool>", # Optional?
    "ScheduleInfo": "Value",
    "Int32[]": "Vec<i32>",
    "TimeIntervalTrigger": "Value",
    "IEnumerable<WebSessionSummary>": "Result<Vec<Value>>",
    "IList<IPermissionsTreeNode>": "Vec<Value>",
    "WebauthnLoginInfo": "Value",
    "IEnumerable<WebauthnCredentialSummary>": "Result<Vec<Value>>",
    "IEnumerable<RunningTask>": "Result<Vec<RunningTask>>",
    "ModuleInfo": "Result<ModuleInfo>",
    "Dictionary<String, Dictionary<String, MethodInfoSummary>>": "HashMap<String, HashMap<String, Value>>",
    "Object": "Value",
    "UpdateInfo": "Result<UpdateInfo>",
    "IEnumerable<ListeningPortSummary>": "Result<Vec<Value>>",
    "Task<JObject>": "Task<Value>",
    "Task<ActionResult<TwoFactorSetupInfo>>": "Task<ActionResult<Value>>",
    "Task<IEnumerable<String>>": "Task<Vec<String>>",
    "Task<UserInfo>": "Task<UserInfo>",
    "Task<IEnumerable<UserInfoSummary>>": "Task<Vec<Value>>",
    "Task<IEnumerable<UserInfo>>": "Task<Vec<UserInfo>>",
    "Task<String>": "Task<String>",
    "Task<AuthRoleSummary>": "Task<Value>",
    "Task<IEnumerable<AuthRoleSummary>>": "Task<Vec<Value>>",
    "Task<IDictionary<Guid, String>>": "Task<HashMap<UUID, Value>>",
    "Task<ActionResult>": "Task<ActionResult<Value>>",
    "Task<ActionResult<Guid>>": "Task<ActionResult<UUID>>",

    ## Custom types
    "Result<Instance>": "Result<Instance>",
    "Result<RemoteTargetInfo>": "Result<RemoteTargetInfo>",
    "SettingsSpec": "SettingsSpec",
    "Status": "Status",
    "Updates": "Updates",
    "Result<HashMap<String, String>>": "Result<HashMap<String, String>>",
    "LoginResult": "LoginResult"
}

custom_types = {
    # API.ADSModule.GetInstance
    "ADSModule.GetInstance": "Result<Instance>",
    # API.ADSModule.GetTargetInfo
    "ADSModule.GetTargetInfo": "Result<RemoteTargetInfo>",

    # API.Core.GetSettingsSpec
    "Core.GetSettingsSpec": "SettingsSpec",
    # API.Core.GetStatus
    "Core.GetStatus": "Status",
    # API.Core.GetUpdates
    "Core.GetUpdates": "Updates",
    # API.Core.GetUserList
    "Core.GetUserList": "Result<HashMap<String, String>>",
    # API.Core.Login
    "Core.Login": "LoginResult",
}

def generate_apimodule_method(module: str, method: str, method_spec: dict):
    # Read the template file
    api_module_method_template = ""
    with open("templates/api_module_method.txt", "r") as tf:
        api_module_method_template = tf.read()
        tf.close()

    # Get the method description
    description = ""
    if "Description" in method_spec.keys():
        description = method_spec["Description"]

    # Get the method parameters
    parameters_docs = ""
    methodParams = method_spec["Parameters"]
    if len(methodParams) > 0:
        parameters_docs += "\n"
    for i in range(len(methodParams)):
        api_module_method_parameter_doc_template = ""
        with open("templates/api_module_method_parameter_doc.txt", "r") as tf:
            api_module_method_parameter_doc_template = tf.read()
            tf.close()

        name = methodParams[i]["Name"]
        type_name = methodParams[i]["TypeName"]

        # Print out the type if it hasn't been added to the type_dict
        if not type_name in type_dict.keys(): print(type_name)

        description = methodParams[i]["Description"]
        optional = methodParams[i]["Optional"]
        if optional == "true": type_name += ", " + optional

        template = api_module_method_parameter_doc_template\
            .replace("%METHOD_PARAMETER_NAME%", name)\
            .replace("%METHOD_PARAMETER_TYPE%", type_dict[type_name])\
            .replace("%METHOD_PARAMETER_DESCRIPTION%", description)\
            .replace("%METHOD_PARAMETER_OPTIONAL%", str(optional))

        parameters_docs += template
    parameters_docs = parameters_docs[:-1]

    # Get the method return type
    return_type = method_spec["ReturnTypeName"]

    # Print out the type if it hasn't been added to the type_dict
    if not return_type in type_dict.keys(): print(return_type)
    return_type = type_dict[return_type]

    # Get the method parameters
    parameters = ""
    for i in range(len(methodParams)):
        name = methodParams[i]["Name"]
        type_name = methodParams[i]["TypeName"]

        # Print out the type if it hasn't been added to the type_dict
        if not type_name in type_dict.keys(): print(type_name)
        parameters += f"{name}: {type_dict[type_name]}, "

    parameters = parameters[:-2]

    # Get the parameters for the data map
    map_string = ""
    if len(methodParams) > 0:
        map_string += "\n"
    for i in range(len(methodParams)):
        api_module_method_parameter_map_template = ""
        with open("templates/api_module_method_parameter_map.txt", "r") as tf:
            api_module_method_parameter_map_template = tf.read()
            tf.close()

        name = methodParams[i]["Name"]
        map_string += api_module_method_parameter_map_template.replace("%METHOD_PARAMETER_NAME%", name)
    map_string = map_string[:-1]

    # Replace placeholders
    template = api_module_method_template\
        .replace("%METHOD_DESCRIPTION%", description)\
        .replace("%METHOD_PARAMETER_DOC%", parameters_docs)\
        .replace("%MODULE_NAME%", module)\
        .replace("%METHOD_NAME%", method)\
        .replace("%METHOD_PARAMETERS%", parameters)\
        .replace("%METHOD_RETURN_TYPE%", return_type)\
        .replace("%METHOD_PARAMETER_MAP%", map_string)

    # End result will return a string
    return template

def generate_apimodule(module: str, methods: dict):
    # Read the template file
    api_module_template = ""
    with open("templates/api_module.txt", "r") as tf:
        api_module_template = tf.read()
        tf.close()

    # Exception for Rust -> make module snake_case
    def snake_case(name: str) -> str:
        if name == "ADSModule": return "ads_module"
        elif name == "RCONPlugin": return "rcon_plugin"
        elif name == "steamcmdplugin": return "steamcmd_plugin"
        return ''.join(['_'+i.lower() if i.isupper() else i for i in name]).lstrip('_')

    # Create a new file called f{module}.java
    f = open(f"../ampapi/src/modules/{snake_case(module)}.rs","w+")
    f.write(api_module_template.replace("%MODULE_NAME%", module))

    for method in methods.keys():
        f.write(generate_apimodule_method(module, method, methods[method]))

    f.write("}\n")
    f.close()

def generate_spec(spec: dict):
    for module in spec.keys():
        if module == "CommonCorePlugin": continue
        generate_apimodule(module, spec[module])

def load_custom_types(spec: dict):
    for type_index in custom_types.keys():
        type_module = type_index.split(".")[0]
        type_method = type_index.split(".")[1]

        # Update the return type
        spec[type_module][type_method]["ReturnTypeName"] = custom_types[type_index]

if __name__ == "__main__":
    # Load remote file
    res = requests.get("https://raw.githubusercontent.com/p0t4t0sandwich/ampapi-spec/main/APISpec.json")
    spec = json.loads(res.content)

    # Load custom types
    load_custom_types(spec)

    generate_spec(spec)
