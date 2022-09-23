use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

//static CONF_PATH: &str = "/etc/moynihan.conf";

static CONF_PATH: &str = "./test/test.conf";
struct ClientInfo {
    // ### /etc/moynihan.conf ###
    // INSTANCE: example.com
    // TOKEN: hogefugapiyo
    // VIS: public
    // ##########################
    instance: String,
    token: String,
    visibility: String,
}

fn get_config(conf_path: &Path) -> ClientInfo {
    let conf_file = match File::open(conf_path) {
        Ok(f)=> f,
        Err(_) => panic!("{:?} does not exist", conf_path),
    };
    let reader = BufReader::new(conf_file);
    let mut conf_vec = Vec::new();

    for v in reader.lines().map(|l| l.unwrap()) {
        conf_vec.push(v);
    }

    let visibility = match conf_vec.pop() {
        Some(s) => s,
        None => panic!("Invalid config file"),
    }.replace("VIS: ", "");
    match visibility.as_str() {
        "public" | "private" => (),
        _ => panic!("Invalid visibility config"),
    }

    let token = match conf_vec.pop() {
        Some(s) => s,
        None => panic!("Invalid config file"),
    }.replace("TOKEN: ", "");

    let instance = match conf_vec.pop() {
        Some(s) => s,
        None => panic!("Invalid config file"),
    }.replace("INSTANCE: ", "");

    ClientInfo{ instance, token, visibility }
}

pub fn toot(msg: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let config: ClientInfo = get_config(Path::new(CONF_PATH));
    let mut map = HashMap::new();
    map.insert("status", msg);
    map.insert("visibility", config.visibility);
    let client = reqwest::blocking::Client::new();
    let api_url = format!("https://{}/api/v1/statuses", config.instance);
    client.post(api_url)
        .header("Authorization", format!("Bearer {}", config.token))
        .json(&map)
        .send()
}

#[cfg(test)]
mod tests {
    use super::*;
    static CONF_PATH: &str = "./test/test.conf";
    #[test]
    fn get_config_test() {
        let conf = get_config(Path::new(CONF_PATH));
        assert_eq!(conf.instance, "example.com");
        assert_eq!(conf.token, "AABBCCDDEE");
        assert_eq!(conf.visibility, "public");
    }

}
