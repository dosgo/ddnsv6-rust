mod ddns;
use getopts::Options;
use std::env;

fn main() {
    println!("ddnsv6 v1.3");
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("D", "ddnstype", "ddnstype [dnspod cloudflare]", "");
    opts.optopt("i", "iptype", "iptype [4,6]", "");
    opts.optopt("d", "domain", "domain", "");
    opts.optopt("s", "subdomain", "subdomain", "");

    opts.optopt("t", "token", "ddns token ", "");
    opts.optopt("e", "email", "cloudflare email", "");
    opts.optopt("a", "apikey", "cloudflare apikey", "");
    opts.optopt("z", "zoneid", "cloudflare zoneid", "");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!(f.to_string())
        }
    };
    let mut ddnstype: String = "cloudflare".to_string();
    if matches.opt_present("D") {
        ddnstype = matches.opt_str("ddnstype").unwrap();
    }
    let mut iptype: String = "6".to_string();
    if matches.opt_present("i") {
        iptype = matches.opt_str("iptype").unwrap();
    }

    let mut domain: String = "".to_string();
    if matches.opt_present("d") {
        domain = matches.opt_str("domain").unwrap();
    }

    let mut subdomain: String = "www".to_string();
    if matches.opt_present("s") {
        subdomain = matches.opt_str("subdomain").unwrap();
    }

    let mut token: String = "".to_string();
    if matches.opt_present("t") {
        token = matches.opt_str("token").unwrap();
    }

    let mut email: String = "".to_string();
    if matches.opt_present("e") {
        email = matches.opt_str("email").unwrap();
    }

    let mut apikey: String = "".to_string();
    if matches.opt_present("a") {
        apikey = matches.opt_str("apikey").unwrap();
    }

    let mut zoneid: String = "".to_string();
    if matches.opt_present("z") {
        zoneid = matches.opt_str("zoneid").unwrap();
    }
    println!("ddnstype:{:}", ddnstype);
    println!("iptype:{:}", iptype);
    println!("domain:{:}", domain);
    println!("subdomain:{:}", subdomain);
    if ddnstype == "dnspod" {
        println!("token:{:}", token);
    } else {
        println!("email:{:}", email);
        println!("apikey:{:}", apikey);
        println!("zoneid:{:}", zoneid);
    }
    if domain == "" {
        println!("domain Not allowed to be empty!");
        return;
    }

    if ddnstype == "cloudflare" && (email == "" || apikey == "" || zoneid == "") {
        println!("Using cloudflare requires email apikey zoneid parameter ");
        return;
    }
    if ddnstype == "ddns" && token == "" {
        println!("Using cloudflare requires token parameter ");
        return;
    }

    ddns::start_ddns_check(
        ddnstype.to_string(),
        iptype.to_string(),
        domain.to_string(),
        subdomain.to_string(),
        token.to_string(),
        email.to_string(),
        apikey.to_string(),
        zoneid.to_string(),
    );
}
