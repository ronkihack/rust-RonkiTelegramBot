extern crate reqwest;
use serde::{Serialize, Deserialize};
use std::{thread, time};
use std::process::Command;
extern crate url;
use url::form_urlencoded;
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

fn test_bot() {
    let test_url = &format!("https://api.telegram.org/{}/getMe", env::var("RONKIBOT_KEY").unwrap());

    let mut res = reqwest::get(test_url).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());
    println!("First read");
    
    let mut text = res.text().unwrap();
    let js: TestBotResponse = serde_json::from_str(&text).unwrap();

    println!("JSON:\n{:#?}", js);
    //println!("Second read");
    //println!("Text:\n{:?}", res.text().unwrap());
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

fn main_loop() {
    let mut last_update = 0;
    let mut last_msg : String = "".to_string();
    let base_req_url = format!("{}/{}/", "https://api.telegram.org", env::var("RONKIBOT_KEY").unwrap());

    //println!("{}", base_req_url);

    //return;

    loop {
        //println!("Last update ID: {}", last_update);
        let mut res = reqwest::get(&format!("{}getUpdates?offset={}", base_req_url, last_update)).unwrap();

        let mut text = res.text().unwrap();
        //println!("{}", text);

        let js: UpdateResponse = serde_json::from_str(&text).unwrap();
        //println!("JSON:\n{:#?}", js);

        match js.result.last() {
            None => last_update = last_update,
            Some(i) => { 
                last_update = i.update_id + 1; 
                let curr_chat_id = i.message.chat.id;
                last_msg = i.message.text.to_string();

                //let output = Command::new(&last_msg).output().unwrap();

                println!("Running command: {}", &last_msg);

                match Command::new("sh")
                                .arg("-c")
                                .arg(&last_msg)
                                .output() {
                    Err(e) => {
                        println!("Error {} in running command: {}", e, last_msg);
                        let encoded: String = form_urlencoded::Serializer::new(String::new())
                            .append_pair("chat_id", &curr_chat_id.to_string())
                            .append_pair("text", &format!("Error {} in running command: {}", e, last_msg))
                            .finish();
                        //println!("{:?}", encoded);

                        //println!("{}", format!("{}sendMessage?{}", base_req_url, encoded));
                        res = reqwest::get(&format!("{}sendMessage?{}", base_req_url, encoded)).unwrap();
                        let mut text = res.text().unwrap();
                        //println!("{}", text);
                    },
                    Ok(out) => {
                        let cmdToRun = String::from_utf8_lossy(&out.stdout);
                        
                        let encoded: String = form_urlencoded::Serializer::new(String::new())
                            .append_pair("chat_id", &curr_chat_id.to_string())
                            .append_pair("text", &cmdToRun.to_string())
                            .finish();
                        //println!("{:?}", encoded);

                        //println!("{}", format!("{}sendMessage?{}", base_req_url, encoded));
                        res = reqwest::get(&format!("{}sendMessage?{}", base_req_url, encoded)).unwrap();
                        let mut text = res.text().unwrap();
                        //println!("{}", text);
                    }
                };

                
            }
        };
        thread::sleep(time::Duration::from_millis(50));
    }

}


fn main() {
    main_loop();
}