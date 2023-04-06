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
            let mut text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;

            let system = "You are a helpful assistant answering questions on Telegram.\n\nIf someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\"";
            let co = ChatOptions {
                // model: ChatModel::GPT4,
                model: ChatModel::GPT35Turbo,
                restart: text.eq_ignore_ascii_case("restart"),
                system_prompt: Some(system),
                retry_times: 3,
            };
            if text.eq_ignore_ascii_case("restart") { text = "Hello"; }
            let c = chat_completion(&openai_key_name, &chat_id.to_string(), &text, &co);
            if let Some(c) = c {
                if c.restarted {
                    _ = tele.send_message(chat_id, "I am starting a new conversation. You can always type \"restart\" to terminate the current conversation.\n\n".to_string() + &c.choice);
                } else {
                    _ = tele.send_message(chat_id, c.choice);
                }
            }
        }
    });
}
