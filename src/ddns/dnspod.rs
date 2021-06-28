use serde_json::Value;
use std::collections::HashMap;

const DNSPOD_API: &str = "https://dnsapi.cn/";

pub struct DnsPod {
    token: String,
}

impl DnsPod {
    pub fn new(value: String) -> DnsPod {
        DnsPod { token: value }
    }
   




    /*同步请求*/
    fn postv1(
        &self,
        cmd: String,
        _params: HashMap<String, String>,
    ) -> Result<HashMap<String, Value>, minreq::Error> {
        let mut params = HashMap::new();
        params.insert("format".to_string(), "json".to_string());
        params.insert("login_token".to_string(), self.token.to_string());
        params.extend(_params);
        let mut url = String::new();
        url.push_str(DNSPOD_API);
        url.push_str(cmd.as_str());


        let mut body = String::new();
        let mut i=0;
        for (key, value) in params.iter() {
            if i>0 {
                body.push('&');
            } 
            body.push_str(&format!("{}={}", key, value));
            i=i+1;
        }
        Ok(minreq::post(url.as_str()).with_body(body).send()?
            .json::<HashMap<String, Value>>()?)
    }
    /*获取请求记录*/
    pub fn get_record(&self, domain: String, record_type: String, sub_domain: String) {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), domain);
        params.insert("sub_domain".to_string(), sub_domain);
        params.insert("record_type".to_string(), record_type);
        let data = self.postv1("Record.List".to_string(), params);

        match data {
            Ok(d) => {
                println!("d:{:?}", d)
            }
            Err(e) => {
                println!("e:{:?}", e);
            }
        }
    }
}
