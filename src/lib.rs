use tg_flows::{listen_to_update, Telegram, UpdateKind};
use openai_flows::{chat_completion, ChatOptions};
use std::env;

#[no_mangle]
pub fn run() {
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let c = chat_completion(&openai_key_name, &msg.chat.id.to_string(), &text, &ChatOptions::default());
            if let Some(c) = c {
                if c.restarted {
                    _ = tele.send_message(msg.chat.id, "Let's start a new conversation!".to_string());
                }
                _ = tele.send_message(msg.chat.id, c.choice);
            }
        }
    });
}
