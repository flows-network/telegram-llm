use tg_flows::{listen_to_update, Telegram, UpdateKind};
use openai_flows::{chat_completion, ChatOptions, ChatModel};
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

            let prompt = "You are a helpful assistant answering questions on Telegram. In your response, you can use simple markdown text to format your answers.\n\n".to_owned() + &text + "\n```";
            let co = ChatOptions {
                model: ChatModel::GPT4,
                restart: text.eq_ignore_ascii_case("restart"),
                restarted_sentence: Some(&prompt)
            };

            let c = chat_completion(&openai_key_name, &msg.chat.id.to_string(), &text, &co);
            if let Some(c) = c {
                if c.restarted {
                    _ = tele.send_message(msg.chat.id, "Let's start a new conversation!".to_string());
                }
                _ = tele.send_message(msg.chat.id, c.choice);
            }
        }
    });
}
