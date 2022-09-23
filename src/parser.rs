use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// 2022/09/22 20:06:53 [error] 1036243#1036243: *3757623 no live upstreams while connecting to upstream
// client: 192.168.11.2
// server:
// request: "GET /hoge HTTP/1.1"
// upstream: "http://localhost/flu/403.html"
// host: "192.168.11.1"
pub struct NginxErrLog {
    date: String,
    time: String,
    //client: std::net::IpAddr,
    client: String,
    payload: String,
}

pub fn parser(log: String) -> NginxErrLog {
    let mut date = "".to_string();
    let mut time = "".to_string();

    for (i, c) in log.chars().enumerate() {
        if i < 10 {
            // Date
            date.push(c);
        }
        if i > 10 && i < 19 {
            // Time
            time.push(c);
        }
    }
    let mut v: Vec<&str> = log.rsplit(',').collect();

    // Error Info
    v.pop();
    // Client
    let client = match v.pop() {
        Some(s) => s,
        None => panic!("Invalid log file"),
    }.to_string().replace(" client: ", "");
    // Server
    let server = match v.pop() {
        Some(s) => s,
        None => panic!("Invalid log file"),
    }.to_string().replace(" server: ", "");
    // Payload
    let payload = match v.pop() {
        Some(s) => s,
        None => panic!("Invalid log file"),
    }.to_string().replace(" request: ", "");

    let parsed_log = NginxErrLog{ date, time, client, payload};
    parsed_log
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn parser_test() {
        let log = String::from("2022/09/22 20:06:54 [error] 1036243#1036243: *3757626 no live upstreams while connecting to upstream, client: 192.168.11.4, server: , request: \"GET /piyo HTTP/1.1\", upstream: \"http://localhost/\", host: \"192.168.11.1\"");
        let parsed_log: NginxErrLog = parser(log);
        assert_eq!("2022/09/22", parsed_log.date);
        assert_eq!("20:06:54", parsed_log.time);
        assert_eq!("192.168.11.4", parsed_log.client);
        assert_eq!("\"GET /piyo HTTP/1.1\"", parsed_log.payload);
    }
}
