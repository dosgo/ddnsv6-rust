mod ddns;
//use async_std::task;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long,default_value = "")]
    token: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt( long, default_value = "")]
    domain: String,

    #[structopt(short, long, default_value = "www")]
    subdomain: String,

    #[structopt(short, long, default_value = "6")]
    iptype: String,

    #[structopt( long, default_value = "cloudflare")]
    ddnstype: String,


    #[structopt(short, long, default_value = "")]
    email: String,

    #[structopt(short, long, default_value = "")]
    apikey: String,

    #[structopt(short, long, default_value = "")]
    zoneid: String,
    
}

fn main() {
    let opt = Opt::from_args();
    println!("ddnsv6 v1.2");
    println!("param:{:#?}", opt);

    if opt.domain == "" {
        println!("domain Not allowed to be empty!");
        return;
    }

    if opt.ddnstype == "cloudflare" && (opt.email == "" || opt.apikey == "" || opt.zoneid == "") {
        println!("Using cloudflare requires email apikey zoneid parameter ");
        return;
    }
    if opt.ddnstype == "ddns" && opt.token == "" {
        println!("Using cloudflare requires token parameter ");
        return;
    }

    ddns::start_ddns_check(
        opt.ddnstype.to_string(),
        opt.iptype.to_string(),
        opt.domain.to_string(),
        opt.subdomain.to_string(),
        opt.token.to_string(),
        opt.email.to_string(),
        opt.apikey.to_string(),
        opt.zoneid.to_string(),
    );
}
