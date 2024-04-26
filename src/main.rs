use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let config = OpenAIConfig::new()
        .with_api_base("https://api.groq.com/openai/v1")
        .with_api_key(api_key);
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model("llama3-70b-8192")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Are you AI?")
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        println!("{:?}", choice.message.content);
    }
    Ok(())
}
