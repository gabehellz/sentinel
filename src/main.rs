use night_sentinel::start_client;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env");
    tracing_subscriber::fmt::init();

    let token =
        std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN environment variable");

    start_client(&token).await.unwrap();
}
