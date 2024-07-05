
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}


pub fn check_ip_address() {
    let ip_address = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in ip_address {
        show_ip(ip)
    }
}

pub fn show_ip(ip: IpAddr) {
    println!("{:?}", ip);
}