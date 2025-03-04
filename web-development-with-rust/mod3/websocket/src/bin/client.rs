use futures::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    // Establish WebSocket connection
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:8080").await.expect("Failed to connect");
    println!("WebSocket connection established");

    // Send a message to the WebSocket server
    let message = Message::text("Hello, server");
    ws_stream.send(message).await.expect("Failed to send message");

    // Receive and print the response from the server
    if let Some(Ok(msg)) = ws_stream.next().await {
        println!("Received message from server: {}", msg);
    }
}