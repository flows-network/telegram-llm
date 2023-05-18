use openai_flows::{chat_completion_default_key, ChatModel, ChatOptions};
use std::env;
use tg_flows::{listen_to_update, Telegram, UpdateKind};

#[no_mangle]
pub fn run() {
    let placeholder_text = env::var("placeholder").unwrap_or("I'm thinking".to_string());
    let telegram_token = env::var("telegram_token").unwrap();

    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let mut text = msg.text().unwrap_or_default();
            let chat_id = msg.chat.id;

            let placeholder = tele
                .send_message(chat_id, placeholder_text)
                .expect("Error occurs when sending Message to Telegram");

            let system = "You are a helpful assistant answering questions on Telegram.\n\nIf someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\"";
            let co = ChatOptions {
                // model: ChatModel::GPT4,
                model: ChatModel::GPT35Turbo,
                restart: text.eq_ignore_ascii_case("restart"),
                system_prompt: Some(system),
                retry_times: 3,
            };
            if text.eq_ignore_ascii_case("restart") {
                text = "Hello";
            }
            let c = chat_completion_default_key(&chat_id.to_string(), text, &co);
            if let Some(c) = c {
                if c.restarted {
                    _ =   tele.edit_message_text(
                        chat_id,
                        placeholder.id,
                        format!("I am starting a new conversation. You can always type \"restart\" to terminate the current conversation.\n\n{}", c.choice)
                    );
                } else {
                    _ = tele.edit_message_text(chat_id, placeholder.id, c.choice);
                }
            }
        }
    });
}
