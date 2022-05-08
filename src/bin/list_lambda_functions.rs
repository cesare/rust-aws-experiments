use aws_sdk_lambda::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let response = client.list_functions().send().await?;
    println!("{:?}", response);
    Ok(())
}
