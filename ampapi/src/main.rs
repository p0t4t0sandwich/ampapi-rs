fn main() {
    let mut API = ampapi::AMPAPI::new("http://172.16.1.172:8080");

    let loginResult = API.Core_Login("".to_string(), "".to_string(), "".to_string(), false);
    
    if loginResult.as_ref().unwrap().get("success").unwrap().is_boolean()
    && loginResult.as_ref().unwrap().get("success").unwrap().as_bool().unwrap() == true {
        let sessionID = loginResult.as_ref().unwrap().get("sessionID").unwrap().as_str().unwrap().to_string();
        API.sessionId = sessionID;
        let statusResult = API.Core_GetStatus();
        println!("{:?}", statusResult);
    } else {
        println!("Login failed");
    }
}

pub mod ampapi {
    use serde_json::Map;
    use serde_json::Value;
    use reqwest::Error;
    use reqwest::blocking;

    pub struct AMPAPI {
        baseUri: String,
        pub sessionId: String,
        dataSource: String
    }

    impl AMPAPI {
        pub fn new(url: &str) -> Self {
            let ds: String;
            if (url.chars().nth(url.len()-1).unwrap()) == '/' {
                ds = format!("{}{}", url, "API");
            } else {
                ds = format!("{}{}", url, "/API");
            }

            Self {
                baseUri: url.to_string(),
                sessionId: "".to_string(),
                dataSource: ds
            }
        }

        pub fn APICall(&self, endpoint: String, args: Map<String, Value>) -> Result<Value, Error> {
            let mut map: Map<String, Value> = Map::new();
            map.insert("SESSIONID".to_string(), Value::String(self.sessionId.to_string()));
            for (key, value) in args.iter() {
                map.insert(key.to_string(), value.clone());
            }

            let client = blocking::Client::new();

            let response = client
                    .post(format!("{}/{}", &self.dataSource, endpoint))
                    .json(&map)
                    .header("Accept", "text/javascript")
                    .header("User-Agent", "AMPAPI-RUST")
                    .send()?
                    .json::<Value>()?;
            Ok(response)
        }

        pub fn Core_Login(&self, username: String, password: String, token: String, rememberMe: bool) -> Result<Value, Error> {
            let mut args: Map<String, Value> = Map::new();
            args.insert("username".to_string(), Value::String(username));
            args.insert("password".to_string(), Value::String(password));
            args.insert("token".to_string(), Value::String(token));
            args.insert("rememberMe".to_string(), Value::Bool(rememberMe));
        
            self.APICall("Core/Login".to_string(), args)
        }

        pub fn Core_GetStatus(&self) -> Result<Value, Error> {
            let args: Map<String, Value> = Map::new();
            self.APICall("Core/GetStatus".to_string(), args)
        }

    }
}
