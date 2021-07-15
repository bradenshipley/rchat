use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use dotenv;

const TWITCH_CLIENT_ID : &str = "TWITCH_CLIENT_ID";
const TWITCH_CLIENT_SECRET : &str = "TWITCH_CLIENT_SECRET";
const TWITCH_CHAT_OAUTH_PASSWORD : &str = "TWITCH_CHAT_OAUTH_PASSWORD";

#[tokio::main]
pub async fn main() {

    // ENV VARIABLES to be used with twitch api
    let _client_id = dotenv::var(TWITCH_CLIENT_ID).unwrap();
    let _client_secret = dotenv::var(TWITCH_CLIENT_SECRET).unwrap();
    let _chat_pass = dotenv::var(TWITCH_CHAT_OAUTH_PASSWORD).unwrap();

    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    // join a channel
    client.join("belkast_".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
