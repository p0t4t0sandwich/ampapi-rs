# ampapi-rs

[![License](https://img.shields.io/github/license/p0t4t0sandwich/ampapi-rs?color=blue)](https://img.shields.io/github/downloads/p0t4t0sandwich/ampapi-rs/LICENSE)
[![Github](https://img.shields.io/github/stars/p0t4t0sandwich/ampapi-rs)](https://github.com/p0t4t0sandwich/ampapi-rs)
[![Github Issues](https://img.shields.io/github/issues/p0t4t0sandwich/ampapi-rs?label=Issues)](https://github.com/p0t4t0sandwich/ampapi-rs/issues)
[![Discord](https://img.shields.io/discord/1067482396246683708?color=7289da&logo=discord&logoColor=white)](https://discord.neuralnexus.dev)

[![Github Releases](https://img.shields.io/github/downloads/p0t4t0sandwich/ampapi-rs/total?label=Github&logo=github&color=181717)](https://github.com/p0t4t0sandwich/ampapi-rs/releases)
[![Crates.io](https://img.shields.io/crates/v/ampapi)](https://crates.io/crates/ampapi)
[![Docs.rs](https://docs.rs/ampapi/badge.svg)](https://docs.rs/ampapi)

An API that allows you to communicate with AMP installations from within Rust.

Documentation for available API calls can be found by appending /API to the URL of any existing AMP installation.

Support:

- Ping `@thepotatoking3452` in the `#development` channel of the [AMP Discord](https://discord.gg/cubecoders)
- My own [development Discord](https://discord.neuralnexus.dev/)

## Installation

```bash
cargo add ampapi
```

## Examples

### CommonAPI Example

```rust
use ampapi::modules::CommonAPI;
use ampapi::types::Status;

fn main() {
    // If you know the module that the instance is using, specify it instead of CommonAPI
    let api = CommonAPI::new(
        String::from("http://localhost:8080/"),
        String::from("admin"),
        String::from("myfancypassword123"),
        "".to_string(),
        "".to_string()
    );

    // API call parameters are simply in the same order as shown in the documentation.
    let _ = api.Core.SendConsoleMessage("say Hello Everyone, this message was sent from the Rust API!".to_string());

    let current_status: Status = api.Core.GetStatus().unwrap();
    let cpu_usage_percent: f64 = current_status.Metrics.get("CPU Usage").unwrap().get("Percent").unwrap().as_f64().unwrap();

    println!("Current CPU usage is: {}%", cpu_usage_percent);
}
```

### Example using the ADS to manage an instance

```rust
use ampapi::modules::{ADS, Minecraft};
use ampapi::types::{IADSInstance, Status};

fn main() {
    let api = ADS::new(
        String::from("http://localhost:8080/"),
        String::from("admin"),
        String::from("myfancypassword123"),
        "".to_string(),
        "".to_string()
    );

    // Get the available instances
    let instances_result = api.ADSModule.GetInstances().unwrap();
    let targets: Vec<IADSInstance> = instances_result.result;

    // In this example, my Hub server is on the second target
    // If you're running a standalone setup, you can just use targets[0]
    // Get the instance ID of the Hub server
    let hub_instance_id = targets[1].AvailableInstances.iter().find(|&x| x.InstanceName == "Hub").unwrap().InstanceID.clone();

    // Use the instance ID to get the API for the instance
    let hub: Minecraft = api.instance_login(hub_instance_id, "Minecraft".to_string()).unwrap().into();

    // Get the current CPU usage
    let current_status: Status = api.Core.GetStatus().unwrap();
    let cpu_usage_percent: f64 = current_status.Metrics.get("CPU Usage").unwrap().get("Percent").unwrap().as_f64().unwrap();

    // Send a message to the console
    let _ = hub.Core.SendConsoleMessage(format!("say Current CPU usage is: {}%", cpu_usage_percent)).unwrap();
}
```

### CommonAPI Example, handling the sessionId and rememberMeToken manually (not recommended)

```rust
fn main() {
    // Under implementation
}
```
