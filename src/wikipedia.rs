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
pub struct Wikipedia {
    pub query: String,
    // pub response: Value,
}

impl Wikipedia {
    pub fn search(query: &str) -> Result<Value, Box<std::error::Error>>{
    // pub fn search(query: &str) -> Result<reqwest::Response, Box<std::error::Error>>{
    // pub fn search(query: &str) -> Result<(), Box<std::error::Error>>{
        // let query = "test";
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        //println!("time_since_epoch={:?}", now);
        let now_in_millis = now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1_000_000;
        let cur_time = (now_in_millis/1000).to_string();
        //println!("current time in sec = {:?}", cur_time);

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

        //println!("{:#?}", params);

        let client = reqwest::Client::new();
        let mut resp = client.post("http://openapi.youdao.com/api")
            .form(&params)
            .send()?;
        // println!("resp = {:#?}", resp);

        let body = resp.text()?.to_string();
        //println!("body = {:#?}", body);

        let json: Value =
            serde_json::from_str(&body).expect("JSON was not well-formatted");

        Ok(json)
    }

    pub fn get(query: &str) -> Result<Value, Box<std::error::Error>>{
        println!("request::post wikipedia");

        let params = [
                        ("action", "query"),
                        ("prop", "info|extracts|pageprops|images"),
                        ("format", "json"),
                        ("inprop", "url"),
                        // ("exlimit", "500"), //limit),
                        ("explaintext", ""),
                        ("exsectionformat", "plain"),
                        ("exchars", "500"), //max_chars),
                        ("exintro", ""),
                        ("redirects", ""),
                        // ("imlimit", "500"),
                        ("generator", "search"),
                        ("gsrsearch", query),
                        ("gsrnamespace", "0"),
                        // ("gsrprop", "score"),
                        // ("gsrinfo", "suggestion"),
                        ("gsrlimit", "8") //limit),
                        ];

        // let resp = reqwest::get("https://httpbin.org/headers")?.text()?;
        // let resp = reqwest::get("http://httpbin.org/range/26")?.text()?;
        // let body = reqwest::get("https://en.jinzhao.wiki/wiki/GNU")?.text()?.to_string();
        let client = reqwest::Client::new();
        let mut resp = client.post("https://en.jinzhao.wiki/w/api.php")
            .form(&params)
            .send()?;
        // println!("resp = {:#?}", resp);

        let body = resp.text()?.to_string();
        // println!("body = {:?}", body);

        let json: Value =
            serde_json::from_str(&body).expect("JSON was not well-formatted");

        // let mut results = vec::new();
        // if json["query"]["pages"].is_object(){
        //     let pages = resp["query"]["pages"].as_object().unwrap();
        //     for (key, value) in pages.iter() {
        //         // println!("{}", value);
        //         // println!("{}", value["extract"]);
        //         // results.push(value["extract"].to_string());
        //         let mut full_text = "".to_string();
        //         full_text = full_text + value["title"] + "\n" + value["extract"] + value["fulurl"];
        //         results.push(value["extract"].to_string());
        //     }
        // }
        // println!("json = {:?}", json);

        Ok(json)
    }
}
