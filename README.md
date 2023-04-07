# <p align="center">A Telegram ChatGPT bot</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

Deploy this function on flows.network, and you will get a Telegram bot that uses ChatGPT to respond to every question in your Telegram DM or channel/group automatically.

<img width="1151" alt="image" src="https://user-images.githubusercontent.com/45785633/226554378-0ea64870-186d-4449-9ae8-d84a2bedf8f6.png">

The example in the above image is to leverage ChatGPT to generate code comments.

## Prerequisites

* You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

* You also need a bot token to access the Telegram API. If you don't already have one, go to Telegram to get a bot token from [@botfather](https://telegram.me/BotFather).


## Deploy a ChatGPT Telegram bot 

To install the ChatGPT Telegram App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Prepare the code: fork this repo

Fork [this repo](https://github.com/flows-network/telegram-gpt) and go to flows.network to deploy your function.

### Deploy the code on flow.network

1. Log into [flows.network](https://flows.network/) from your GitHub account. It's free.

2. Click on the "Create a Flow" button to start deploying the ChatGPT GitHub APP

3. Authenticate the flows.network to access the `telegram-gpt` repo you just forked.
![image](https://user-images.githubusercontent.com/45785633/226558160-7a319520-2212-41e4-b40e-43ca5f8d5712.png)

4. Click the Advanced test to see more settings. Here we need to use Environment Variables to pass the Telegram token and OpenAI API key name. 
* `telegram_token`: Fill in the token you received from Fatherbot.
* `openai_key_name`: Fill in the name you want for your OpenAI key. You can put any name here, and we will connect it to the actual key later.

![image](https://user-images.githubusercontent.com/45785633/226562489-ff140061-d1e4-44ab-8cc9-369983cb016d.png)

5. Click on the Deploy button to deploy your function.

## Configure SaaS integrations

After that, the [flows.network](https://flows.network/) will direct you to configure the SaaS integration required by your flow. Since we have configured Telegram in the above step, OpenAI is the only SaaS we need to configure here. 

![image](https://user-images.githubusercontent.com/45785633/226564674-902933b5-8ff3-4724-93e3-2b2f67dc0b9a.png)

Click the "Connect/+ Add new authentication" button to add your OpenAI API key. On the next page, copy and paste your OpenAI API key and then name the key. **Note** the name you enter here should be the same as the name in the environment variable.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

## Give it a try. 

Click on the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status becomes `running`, the Telegram ChatGPT App goes live. Go ahead and send a private message to the bot! You can also invite this bot to your channel/group.

## Dev notes

If you want to build locally, make sure you have installed Rust and added the `wasm32-wasi` target.

```
cargo build --target wasm32-wasi --release
```

