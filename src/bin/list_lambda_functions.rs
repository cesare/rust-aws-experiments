use aws_sdk_lambda::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env();
    let response = client.list_functions().send().await?;
    println!("{:?}", response);
    Ok(())
}
