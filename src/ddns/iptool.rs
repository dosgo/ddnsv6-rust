use cidr_utils::cidr::IpCidr;
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
        if IsPublicIP(iface.addr.ip()) {
            return iface.addr.ip().to_string();
        }
        println!("{:#?}", iface.addr.ip());
    }

    return "".to_string();
}

fn isPrivateIPv4(ip: IpAddr) -> bool {
    let cidr1 = IpCidr::from_str("10.0.0.0/8").unwrap();
    let cidr2 = IpCidr::from_str("172.16.0.0/12").unwrap();
    let cidr3 = IpCidr::from_str("192.168.0.0/16").unwrap();
    return cidr1.contains(ip) || cidr2.contains(ip) || cidr3.contains(ip);
}

fn isPrivateIPv6(ip: IpAddr) -> bool {
    let cidr1 = IpCidr::from_str("fc00::/7").unwrap();
    return cidr1.contains(ip);
}

//IsPublicIP 判断是否公网IP,支持IPv4,IPv6
fn IsPublicIP(ip: IpAddr) -> bool {
    // IPv4私有地址空间
    // A类：10.0.0.0到10.255.255.255
    // B类：172.16.0.0到172.31.255.255
    // C类：192.168.0.0到192.168.255.255
    if ip.is_ipv4() {
        return !isPrivateIPv4(ip);
    }
    // IPv6私有地址空间：以前缀FEC0::/10开头
    if ip.is_ipv6() {
        return !isPrivateIPv6(ip);
    }
    return false;
}
