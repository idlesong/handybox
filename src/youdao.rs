// This will POST a body of `foo=bar&baz=quux`
extern crate crypto;
extern crate serde;
extern crate serde_json;
use std::time::{SystemTime, UNIX_EPOCH};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
// use std::collections::HashMap;
use serde_json::{Value};

#[derive(Clone)]
pub struct Youdao {
    pub query: String,
    // pub response: Value,
}

impl Youdao {
    pub fn search(query: &str) -> Result<Value, Box<std::error::Error>>{
    // pub fn search(query: &str) -> Result<reqwest::Response, Box<std::error::Error>>{
    // pub fn search(query: &str) -> Result<(), Box<std::error::Error>>{
        // let query = "test";
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        println!("time_since_epoch={:?}", now);
        let now_in_millis = now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1_000_000;
        let cur_time = (now_in_millis/1000).to_string();
        println!("current time in sec = {:?}", cur_time);

        let APP_KEY = String::from("425556768091f696");
        let APP_SECRET = "A8a6xv4nWDvgBmpeeHOgpInqsY8A42n4";
        let salt = now_in_millis.to_string();
        let sign_str = APP_KEY.clone() + query + &salt + &cur_time + APP_SECRET;
        let mut hasher = Sha256::new();
        hasher.input_str(&sign_str);
        let sign = hasher.result_str();

        let params = [
                        ("from", "EN"),
                        ("to", "zh-CHS"),
                        ("signType", "v3"),
                        ("curtime", &cur_time),
                        ("appKey", &APP_KEY),
                        ("q", &query),
                        ("salt", &salt),
                        ("sign", &sign)];

        println!("{:#?}", params);

        let client = reqwest::Client::new();
        let mut resp = client.post("http://openapi.youdao.com/api")
            .form(&params)
            .send()?;
        // println!("resp = {:#?}", resp);

        let body = resp.text()?.to_string();
        // println!("body = {:#?}", body);

        let json: Value =
            serde_json::from_str(&body).expect("JSON was not well-formatted");
        println!("json = {:#?}", json);

        Ok(json)
    }
}
