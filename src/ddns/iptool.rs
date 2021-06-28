use std::net::IpAddr;
/*
*获取本机公网IP
*ipver 4 ipv4  6 ipv6
 */
pub fn get_public_ip(ipver: String) -> String {
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        if iface.addr.is_loopback() {
            continue;
        }
        if ipver == "6" && iface.addr.ip().is_ipv4() {
            continue;
        }
        if ipver == "4" && iface.addr.ip().is_ipv6() {
            continue;
        }
        if IsPublicIPV1(iface.addr.ip()) {
            return iface.addr.ip().to_string();
        }
        println!("{:#?}", iface.addr.ip());
    }

    return "".to_string();
}

//IsPublicIP 判断是否公网IP,支持IPv4,IPv6
pub fn IsPublicIPV1(ip: IpAddr) -> bool {
    // IPv4私有地址空间
    // A类：10.0.0.0到10.255.255.255
    // B类：172.16.0.0到172.31.255.255
    // C类：192.168.0.0到192.168.255.255
    if ip.is_ipv4() {
        let _ip = ip.to_string();
        let ip = &_ip[..];
        let split = _ip.split(".");
        let ips: Vec<&str> = split.clone().collect();

        if ip.starts_with("10.") {
            return false;
        }
        if ip.starts_with("172.") {
            if split.count() > 1 {
                let ipTwo = ips[1].parse::<i32>().unwrap(); //String to int
                if ipTwo >= 16 && ipTwo <= 31 {
                    return false;
                }
            }
        }
        if ip.starts_with("192.168") {
            return false;
        }
    }
    // IPv6私有地址空间：以前缀FEC0::/10开头
    if ip.is_ipv6() {
        let _ip = ip.to_string().to_ascii_lowercase();
        let ip = &_ip[..];
        if ip.starts_with("fec0") {
            return false;
        }
    }
    return true;
}
