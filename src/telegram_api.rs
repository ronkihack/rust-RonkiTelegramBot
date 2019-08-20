extern crate reqwest;
use serde::{Serialize, Deserialize};
use std::env;
use std::{thread, time};

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
    pub message_id : u32,
    pub from: FromDetails,
    pub chat: ChatDetails,
    pub date: u32,
    pub text: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResult {
    pub update_id : u32,
    pub message : MsgResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDetails {
    pub id : u32,
    pub first_name: String,
    pub last_name: String,
    pub r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDetails {
    pub id : u32,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: String,
    pub language_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub ok: bool,
    pub result: Vec<UpdateResult>,
}

struct GetReqParam (String, String);

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

pub struct TelegramAPI {

}


impl TelegramAPI {
    pub fn init(callback_func: fn(u32, &String)) {

        thread::spawn(move || {
            let mut next_update = 0;
            loop {
                let get_updates_url = build_get_req_url(    "https://api.telegram.org", 
                                                            "getUpdates", 
                                                            &[GetReqParam("offset".to_string(), next_update.to_string())]);

                let mut res = reqwest::get(&get_updates_url).unwrap();

                let text = res.text().unwrap();

                let json_obj : UpdateResponse = serde_json::from_str(&text).unwrap();

                for response in json_obj.result {
                    next_update = response.update_id + 1;
                    callback_func(response.message.chat.id, &response.message.text);
                }

                thread::sleep(time::Duration::from_millis(50));
            }
        });
    }

    pub fn send_msg(chat_id : u32, text: &str) {
        let get_updates_url = build_get_req_url("https://api.telegram.org", 
                                                "sendMessage", 
                                                &[  GetReqParam("chat_id".to_string(), chat_id.to_string()),
                                                    GetReqParam("text".to_string(), text.to_string())]);

        reqwest::get(&get_updates_url).unwrap();
    }

}