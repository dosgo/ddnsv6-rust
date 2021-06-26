mod ddns;
use clap::{App, Arg};
//use async_std::task;

fn main() {
    println!("ddnsv6 v1.0");
    let matches = App::new("ddnsv6")
        .version("1.0")
        .author("dosgo@qq.com")
        .about("")
        .arg(
            Arg::with_name("token")
                .long("token")
                .value_name("token")
                .help("dnspod token"),
        )
        .arg(
            Arg::with_name("domain")
                .long("domain")
                .value_name("domain")
                .help("domain"),
        )
        .arg(
            Arg::with_name("subdomain")
                .long("subdomain")
                .value_name("subdomain")
                .help("sub domain"),
        )
        .arg(
            Arg::with_name("iptype")
                .long("iptype")
                .value_name("iptype")
                .help("ip ver [4,6]"),
        )
        .arg(
            Arg::with_name("ddnstype")
                .long("ddnstype")
                .value_name("ddnstype")
                .help("ddns type dnspod or cloudflare."),
        )
        .arg(
            Arg::with_name("email")
                .long("email")
                .value_name("email")
                .help("cloudflare email."),
        )
        .arg(
            Arg::with_name("apikey")
                .long("apikey")
                .value_name("apikey")
                .help("cloudflare apikey."),
        )
        .arg(
            Arg::with_name("zoneid")
                .long("zoneid")
                .value_name("zoneid")
                .help("cloudflare zoneid."),
        )
        .get_matches();
    let token = matches.value_of("token").unwrap_or("");
    println!("token : {}", token);
    let domain = matches.value_of("domain").unwrap_or("");
    println!("domain : {}", domain);
    let subdomain = matches.value_of("subdomain").unwrap_or("www");
    println!("subdomain : {}", subdomain);
    let iptype = matches.value_of("iptype").unwrap_or("6");
    println!("iptype : {}", iptype);
    let ddnstype = matches.value_of("ddnstype").unwrap_or("cloudflare");
    println!("ddnstype : {}", ddnstype);
    let email = matches.value_of("email").unwrap_or("");
    println!("cloudflare email : {}", email);
    let apikey = matches.value_of("apikey").unwrap_or("");
    println!("cloudflare apikey : {}", apikey);
    let zoneid = matches.value_of("zoneid").unwrap_or("");
    println!("cloudflare zoneid : {}", zoneid);

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
