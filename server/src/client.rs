use rocket::form::error::Entity::Value;
use rocket::futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub struct Client {

}

impl Client {
    pub async fn start(
        id: u32,
        stream: TcpStream,
        broadcast: Sender<String>,
    ) -> Sender<String> {
        let (sender, mut rx) = mpsc::channel::<String>(100);

        let mut socket = accept_async(stream)
            .await
            .expect("Failed to accept as websocket");

        tokio::spawn(async move {
            println!("{} Connected", id);

            loop {
                println!("{} Loop", id);

                tokio::select! {
                    v = socket.next() => {
                        println!("{} Received", id);
                        let Some(message) = v else {
                            println!("{} Disconnecting", id);
                            break;
                        };

                        let message: Message = message.unwrap();

                        match message {
                            Message::Text(string) => {
                                broadcast.send(string).await
                                    .expect("Ooops");
                            },
                            _ => println!("Unsupported message type"),
                        }
                    },

                    v = rx.recv() => {
                        println!("{} Sending message", id);
                        let Some(message) = v else {
                            break;
                        };

                        socket.send(Message::Text(message)).await;
                    },
                }
            }
        });

        sender
    }
}
