//use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use core::fmt;
use std::fmt::Debug;


// 2022/09/22 20:06:53 [error] 1036243#1036243: *3757623 no live upstreams while connecting to upstream
// client: 192.168.11.2
// server:
// request: "GET /hoge HTTP/1.1"
// upstream: "http://localhost/flu/403.html"
// host: "192.168.11.1"
pub struct NginxErrLog {
    pub date: String,
    pub time: String,
    //client: std::net::IpAddr,
    pub client: String,
    pub payload: String,
}

pub enum NginxParserErr {
    LogIsNotice,
    LogIsCritical,
    InvalidLogFile,
}
impl fmt::Display for NginxParserErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            NginxParserErr::LogIsNotice => write!(f, "Notice detected at Nginx Error Log"),
            NginxParserErr::LogIsCritical => write!(f, "Critical detected at Nginx Error Log"),
            NginxParserErr::InvalidLogFile => write!(f, "Invalid Nginx Error Log Detected"),
        }
    }
}

pub type ParserResult = Result<NginxErrLog, NginxParserErr>;

pub fn parser(log: String) -> ParserResult {
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
    let info = match v.pop() {
        Some(s) => s,
        None => return Err(NginxParserErr::InvalidLogFile),
    };
    //// if error is notice
    match info.find("[notice]") {
        Some(_) => return Err(NginxParserErr::LogIsNotice),
        None => (),
    }
    //// if error is critical
    match info.find("[crit]") {
        Some(_) => return Err(NginxParserErr::LogIsCritical),
        None => (),
    }

    // Client
    let client = match v.pop() {
        Some(s) => s,
        None => return Err(NginxParserErr::InvalidLogFile),
    }.to_string().replace(" client: ", "");
    // Server (not being used now)
    v.pop();
    // Payload
    let payload = match v.pop() {
        Some(s) => s,
        None => return Err(NginxParserErr::InvalidLogFile),
    }.to_string().replace(" request: ", "");

    Ok(NginxErrLog{ date, time, client, payload})
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn parser_normal_log() {
        let test_log = String::from("2022/09/22 20:06:54 [error] 1036243#1036243: *3757626 no live upstreams while connecting to upstream, client: 192.168.11.4, server: , request: \"GET /piyo HTTP/1.1\", upstream: \"http://localhost/\", host: \"192.168.11.1\"");
        let parsed_log: NginxErrLog = match parser(test_log) {
            Ok(parsed_log) => parsed_log,
            Err(e) => panic!("{}", e),
        };
        assert_eq!("2022/09/22", parsed_log.date);
        assert_eq!("20:06:54", parsed_log.time);
        assert_eq!("192.168.11.4", parsed_log.client);
        assert_eq!("\"GET /piyo HTTP/1.1\"", parsed_log.payload);
    }
    #[test]
    fn parser_expect_log() {
        let notice_log = String::from("2022/09/24 03:20:53 [notice] 1117565#1117565: signal process started");
        let crit_log = String::from("2022/10/04 12:31:15 [crit] 1654810#1654810: *2021764 SSL_do_handshake() failed (SSL: error:141CF06C:SSL routines:tls_parse_ctos_key_share:bad key share) while SSL handshaking, client: 192.168.2.1, server: 0.0.0.0:443");
    }
}
