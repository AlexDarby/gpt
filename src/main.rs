use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    key: String,

    #[arg(short, long)]
    message: String,
}


#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    // Getting the API key here
    let key = args.key;

    // Getting the API key here
    let message = args.message;

    // Creating a new ChatGPT client.
    let client = ChatGPT::new(key)?;

    // Sending a message and getting the completion
    let response: CompletionResponse = client
        .send_message(message)
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
