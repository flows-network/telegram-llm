use tg_flows::{listen_to_update, Telegram, Update, UpdateKind};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    logger::init();
    let telegram_token = std::env::var("telegram_token").unwrap();
    let placeholder_text = std::env::var("placeholder").unwrap_or("Typing ...".to_string());

    listen_to_update(&telegram_token, |update| {
        let tele = Telegram::new(telegram_token.to_string());
        handler(tele, &placeholder_text, update)
    }).await;

    Ok(())
}

async fn handler(tele: Telegram, placeholder_text: &str, update: Update) {
    if let UpdateKind::Message(msg) = update.kind {
        let chat_id = msg.chat.id;
        log::info!("Received message from {}", chat_id);
        let placeholder = tele
            .send_message(chat_id, placeholder_text)
            .expect("Error occurs when sending Message to Telegram");

        let mut text = msg.text().unwrap_or("");
        let system = "You are a helpful assistant answering questions on Telegram.\n\nIf someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\"";
        let mut openai = OpenAIFlows::new();
        openai.set_retry_times(3);
        let co = ChatOptions {
            // model: ChatModel::GPT4,
            model: ChatModel::GPT35Turbo,
            restart: text.eq_ignore_ascii_case("restart"),
            system_prompt: Some(system),
        };
        if text.eq_ignore_ascii_case("restart") { text = "Hello"; }
        match openai.chat_completion(&chat_id.to_string(), &text, &co).await {
            Ok(r) => {
                if r.restarted {
                    log::info!("Restart converstion for {}", chat_id);
                    _ = tele.edit_message_text(chat_id, placeholder.id, "I am starting a new conversation. You can always type \"restart\" to terminate the current conversation.\n\n".to_string() + &r.choice);
                } else {
                    _ = tele.edit_message_text(chat_id, placeholder.id, r.choice);
                }
            }
            Err(e) => {
                log::error!("OpenAI returns error: {}", e);
            }
        }
    }
}
