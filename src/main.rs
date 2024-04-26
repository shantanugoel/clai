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

fn get_system_prompt() -> String {
    let system_info = format!("OS: {}, Arch: {}", env::consts::OS, env::consts::ARCH,);
    format!("
    You are clai, a command line code snippet generator. 
    Given the user's desired outcome, respond with a helpful command line code or command to gnerate the desired outcome.
    Keep the output limited to just the required code and a one short sentence describing it. This one short sentence description
    should be the first line in the output.
    If the user asks for a specific language or framework, provide the code or command snippet in that language/framework.
    If the code or command snippet is multiple lines, separate each line with a newline character.
    Do not write any markdown, html or any other programming language code except various shell languages.
    Make sure the code or command snippets provided are in line with the system info given here and the common shells
    that are used on that system. System Info: {}
    If the code or command snippet requires the use of a particular shell, mention the shell name.
    Example output:
    Prints current date and time in the bash shell.
    date
    ", system_info)
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let config = OpenAIConfig::new()
        .with_api_base("https://api.groq.com/openai/v1")
        .with_api_key(api_key);
    let client = Client::with_config(config);

    let prompt = env::args().collect::<Vec<_>>().join(" ");
    let request = CreateChatCompletionRequestArgs::default()
        .model("llama3-70b-8192")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(get_system_prompt())
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    let binding: String = response.choices[0].clone().message.content.unwrap();
    let lines = binding.split('\n');
    let mut first = true;
    for line in lines {
        if first {
            println!("{}", line);
            first = false;
        } else {
            println!("$ {}", line);
        }
    }
    Ok(())
}
