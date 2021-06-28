pub mod cloudflare;
pub mod dnspod;
pub mod iptool;

use std::{thread, time};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
pub fn checkip(iptype: String, caddr: String, domain: String) -> bool {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let _response = resolver.lookup_ip(domain.as_str());
    match _response {
        Ok(response) => {
            for ans in response.iter() {
                if iptype == "4" && ans.is_ipv4() {
                    if caddr == ans.to_string() {
                        return true;
                    }
                }
                if iptype == "6" && ans.is_ipv6() {
                    if caddr == ans.to_string() {
                        return true;
                    }
                }
            }
        }
        Err(e) => {
            println!("e:{:?}", e);
        }
    }
    return false;
}

fn modify(
    email: String,
    apikey: String,
    zoneid: String,
    dnsPodToken: String,
    ddnstype: String,
    domain: String,
    value: String,
    sub_domain: String,
    record_type: String,
) -> bool {
    let dns_pod: dnspod::DnsPod;
    let cloud_dns: cloudflare::CloudFlare;
    if ddnstype == "ddns" {
        dns_pod = dnspod::DnsPod::new(dnsPodToken);
        //  dns_pod.modify(domain.clone(), value, sub_domain.clone(), record_type);
    } else {
        cloud_dns = cloudflare::CloudFlare::new(email, apikey, zoneid);
        return cloud_dns.modify(domain.clone(), value, sub_domain.clone(), record_type);
    }

    return false;
}

pub fn start_ddns_check(
    ddnstype: String,
    iptype: String,
    domain: String,
    sub_domain: String,
    dnsPodToken: String,
    email: String,
    apikey: String,
    zoneid: String,
) {
    let mut full_domain = String::new();
    full_domain.push_str(sub_domain.clone().as_str());
    full_domain.push_str(".");
    full_domain.push_str(domain.clone().as_str());

    while true {
        println!("checkIpUpdateing....");
        if iptype == "4" || iptype == "0" {
            let ipv4 = iptool::get_public_ip("4".to_string());
            if ipv4 != "" {
                println!("find ipv4 address :{:?}", ipv4);
                if checkip("4".to_string(), ipv4.clone(), full_domain.clone()) == false {
                    let modifyRes = modify(
                        email.clone(),
                        apikey.clone(),
                        zoneid.clone(),
                        dnsPodToken.clone(),
                        ddnstype.clone(),
                        domain.clone(),
                        ipv4,
                        sub_domain.clone(),
                        "A".to_string(),
                    );
                    println!("modifyRes:{:?}", modifyRes);
                } else {
                    println!("No changes")
                }
            }
        }
        if iptype == "6" || iptype == "0" {
            let ipv6 = iptool::get_public_ip("6".to_string());
            if ipv6 != "" {
                println!("find ipv6 address :{:}", ipv6);
                //Modify  update
                if checkip("6".to_string(), ipv6.clone(), full_domain.clone()) == false {
                    let modifyRes = modify(
                        email.clone(),
                        apikey.clone(),
                        zoneid.clone(),
                        dnsPodToken.clone(),
                        ddnstype.clone(),
                        domain.clone(),
                        ipv6,
                        sub_domain.clone(),
                        "AAAA".to_string(),
                    );
                    println!("modifyRes:{:?}", modifyRes);
                } else {
                    println!("No changes")
                }
            }
        }
        thread::sleep(time::Duration::from_secs(60));
    }
}
