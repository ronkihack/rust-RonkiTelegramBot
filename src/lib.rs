extern crate reqwest;
use serde::{Serialize, Deserialize};
use std::{thread, time};
use std::process::Command;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestBotResultResponse {
    pub id : u32,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestBotResponse {
     pub ok: bool,
     pub result: TestBotResultResponse
 }

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgResult {
    message_id : u32,
    from: FromDetails,
    chat: ChatDetails,
    date: u32,
    text: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResult {
    update_id : u32,
    message : MsgResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDetails {
    id : u32,
    first_name: String,
    last_name: String,
    r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDetails {
    id : u32,
    is_bot: bool,
    first_name: String,
    last_name: String,
    language_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub ok: bool,
    pub result: Vec<UpdateResult>,
}

pub struct GetReqParam (String, String);

fn build_get_req_url(base_address: &str, api_name: &str, get_data: &[GetReqParam]) -> String {
    let mut ret = format!("{}/{}/{}?", base_address, env::var("RONKIBOT_KEY").unwrap(), api_name);

    let mut pair_string = String::new();

    for pair in get_data {
        pair_string.push_str(&format!("{}={}&", pair.0, pair.1));
    }
    if get_data.len() != 0 {
        pair_string.truncate(pair_string.len() - 1);
    }

    //in order to show line breaks in telegram
    pair_string = pair_string.replace("\n", "%0A");

    ret.push_str(&pair_string);

    ret
}

pub fn req_updates(last_update : u32) -> UpdateResponse {
    let get_updates_url = build_get_req_url(    "https://api.telegram.org", 
                                                "getUpdates", 
                                                &[GetReqParam("offset".to_string(), last_update.to_string())]);

    let mut res = reqwest::get(&get_updates_url).unwrap();

    let text = res.text().unwrap();

    serde_json::from_str(&text).unwrap()
}

pub fn send_msg(chat_id : u32, text: &str) {
    let get_updates_url = build_get_req_url(    "https://api.telegram.org", 
                                                "sendMessage", 
                                                &[  GetReqParam("chat_id".to_string(), chat_id.to_string()),
                                                    GetReqParam("text".to_string(), text.to_string())]);

    reqwest::get(&get_updates_url).unwrap();
}

pub fn main_loop() {
    let mut last_update = 0;

    loop {

        let update_resp = req_updates(last_update);

        if let None = update_resp.result.last() {
            continue;
        }

        for resp in update_resp.result {
            last_update = resp.update_id + 1; 
            let curr_chat_id = resp.message.chat.id;
            let last_msg = resp.message.text.to_string();

            println!("Running command: {}", &last_msg);

            match Command::new("sh")
                            .arg("-c")
                            .arg(&last_msg)
                            .output() {
                Err(e) => {
                    println!("Error {} in running command: {}", e, last_msg);
                    send_msg(curr_chat_id, &format!("Error {} in running command: {}", e, last_msg));
                },
                Ok(out) => {
                    send_msg(curr_chat_id, &String::from_utf8_lossy(&out.stdout));
                }
            };
        }

        thread::sleep(time::Duration::from_millis(50));
    }

}