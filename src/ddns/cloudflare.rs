
use serde_json::Number;
use serde_json::Value;
use std::collections::HashMap;
const CLOUD_FLARE_API: &str = "https://api.cloudflare.com/client/v4/";
//const cloudFlareApi: &str = "http://192.168.8.229/hall-main/";

pub struct CloudFlare {
    email: String,
    apikey: String,
    zoneid: String,
}

impl CloudFlare {
    pub fn new(_email: String, _apikey: String, _zoneid: String) -> CloudFlare {
        CloudFlare {
            email: _email,
            apikey: _apikey,
            zoneid: _zoneid,
        }
    }

   
    /*同步请求*/
    fn postv1(
        &self,
        cmd: String,
        params: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, minreq::Error> {
        let mut url = String::new();
        url.push_str(CLOUD_FLARE_API);
        url.push_str(cmd.as_str());

        Ok(minreq::post(url)
        .with_header("Content-Type", "application/json")
        .with_header("X-Auth-Email", self.email.as_str())
        .with_header("X-Auth-Key", self.apikey.as_str())
        .with_json(&params)?
        .send()?
        .json::<HashMap<String, Value>>()?)
    }

    

    /*同步请求*/
    fn getv1(&self, cmd: String)-> Result<HashMap<String, Value>, minreq::Error>{
        let mut url = String::new();
        url.push_str(CLOUD_FLARE_API);
        url.push_str(cmd.as_str());
 
        Ok(minreq::get(url)
        .with_header("Content-Type", "application/json")
        .with_header("X-Auth-Email", self.email.as_str())
        .with_header("X-Auth-Key", self.apikey.as_str())
        .send()?.json::<HashMap<String, Value>>()?)

    }

    /*获取请求记录*/
    pub fn get_domain_id(&self, domain: String) -> String {
        let mut cmd = String::new();
        cmd.push_str("zones/");
        cmd.push_str(self.zoneid.as_str());
        cmd.push_str("/dns_records?name=");
        cmd.push_str(domain.as_str());
        let result = self.getv1(cmd);
        match result {
            Ok(data) => {
                println!("v:{:?}", data);
                match data.get("success") {
                    Some(v1) => {
                        println!("v1:{:?}", v1);
                        if v1.as_bool() == Some(true) {
                            match data.get("result") {
                                Some(v2) => {
                                    println!("v2:{:?}", v2);
                                    for val in v2.as_array().unwrap().iter() {
                                        let obj = val.as_object().unwrap();
                                        println!("value is :{}", val);
                                        if obj["name"].as_str() == Some(domain.as_str()) {
                                            return obj["id"].to_string();
                                        }
                                    }
                                }
                                None => {
                                    println!("没有找到对应值。");
                                }
                            }
                        }
                    }
                    None => {
                        println!("没有找到对应值。");
                    }
                }
            }
            Err(e) => {
                println!("e:{:?}", e);
                return "".to_string();
            }
        };
        return "".to_string();
    }

    pub fn update_dns(&self, domain: String, ip: String, _type: String, ttl: u32) -> bool {
        let domainid = self.get_domain_id(domain.clone());
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("type".to_string(), Value::String(_type));
        params.insert("name".to_string(), Value::String(domain.clone()));
        params.insert("content".to_string(), Value::String(ip));
        params.insert("ttl".to_string(), Value::Number(Number::from(ttl)));

        let mut cmd = String::new();
        cmd.push_str("zones/");
        cmd.push_str(self.zoneid.as_str());
        cmd.push_str("/dns_records");
        cmd.push_str(domain.as_str());
        if domainid != "" {
            cmd.push_str("/");
            cmd.push_str(domainid.as_str());
        }
        let result = self.postv1(cmd, params);
        match result {
            Ok(data) => match data.get("success") {
                Some(v1) => {
                    if v1.as_bool() == Some(true) {
                        return true;
                    }
                }
                None => {}
            },
            Err(e) => {}
        }
        return false;
    }

    pub fn modify(
        &self,
        _domain: String,
        value: String,
        sub_domain: String,
        record_type: String,
    ) -> bool {
        let mut domain = String::new();
        domain.push_str(sub_domain.as_str());
        domain.push_str(".");
        domain.push_str(_domain.as_str());
        return self.update_dns(domain, value, record_type, 120);
    }
}
