use std::{
    net::SocketAddr,
    error::Error,
};

use futures_channel::mpsc::{unbounded};
use futures_util::{future, stream::TryStreamExt, StreamExt, pin_mut};
use tokio::net::{TcpListener, TcpStream};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GenericJSONRequest {
    #[serde(rename = "id")]
    data_id: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("now running dforum websockets on :8084");

    let listener = TcpListener::bind("127.0.0.1:8084").await?;
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);


    // Insert the write part of this peer to the peer map.
    let (mut tx, rx) = unbounded();

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());

        let j = serde_json::from_str::<GenericJSONRequest>(msg.to_string().as_str());
        match j {
            Ok(a) => {
                tx.start_send(msg);
            },
            Err(err) => {
                tx.start_send(err.to_string().into());
            }
        }
        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
}