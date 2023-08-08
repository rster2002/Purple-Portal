use std::time::Duration;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use crate::modules::ws_client::error::WsError;
use crate::modules::ws_client::models::ws_messages::{IncomingMessage, OutgoingMessage};
use futures_util::stream::StreamExt;
use tokio_tungstenite::tungstenite::Message;
use crate::PurplePortalServer;

pub mod models;
pub mod error;

const WS_TIMEOUT: u64 = 1000;

pub struct WsClient {
    ws_stream: WebSocketStream<TcpStream>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> Result<Self, WsError> {
        let ws_stream = accept_async(stream)
            .await?;

        let client = WsClient {
            ws_stream,
        };

        Ok(client)
    }

    /// Runs the websocket loop. This function will return `Ok(())` when the socket closes
    /// gratefully and will return an Err if something happens that could not be recovered from.
    async fn run_loop(&mut self) -> Result<(), WsError> {
        todo!()
        // let authentication_message = self.receive_with_timeout()
        //     .await?;
        //
        // let IncomingMessage::Authenticate { password, client_info } = authentication_message else {
        //     return Err(OutgoingMessage::Conflict.into());
        // };
        //
        // if password != "abc".to_string() {
        //     return Err(OutgoingMessage::AuthenticationFailed.into());
        // }
        //
        // self.send(OutgoingMessage::AuthenticationSuccess)
        //     .await?;

        // loop {
        //     tokio::select! {
        //         v = self.receive() => {
        //             // if let Err(ws_error) = v {
        //             //     if let WsError::OutgoingError(outgoing) = ws_error {
        //             //         self.send(outgoing)
        //             //             .await?;
        //             //
        //             //         continue;
        //             //     } else {
        //             //         return Err(ws_error);
        //             //     }
        //             // }
        //         },
        //     }
        // }
    }

    // pub async fn send(&mut self, message: OutgoingMessage) -> Result<(), WsError> {
    //     let json_string = serde_json::to_string(&message)?;
    //
    //     self.ws_stream.send(Message::Text(json_string))
    //         .await?;
    //
    //     Ok(())
    // }
    //
    // /// Receive and wait on the next message of the client.
    // pub async fn receive(&mut self) -> Result<IncomingMessage, WsError> {
    //     let message = self.ws_stream.next()
    //         .await
    //         .ok_or(WsError::SocketClosed)??;
    //
    //     let Message::Text(string) = message else {
    //         return Err(OutgoingMessage::IncorrectFormat.into());
    //     };
    //
    //     let incoming_message = match serde_json::from_str(&string) {
    //         Ok(message) => message,
    //         Err(error) => {
    //             return Err(OutgoingMessage::FailedToParseJson(error.to_string()).into());
    //         },
    //     };
    //
    //     Ok(incoming_message)
    // }
    //
    // /// Receive the next message from the client. If the client did not send something within
    // /// the expected time frame, it returns an error with an [OutgoingMessage::Timeout].
    // pub async fn receive_with_timeout(&mut self) -> Result<IncomingMessage, WsError> {
    //     tokio::select! {
    //         v = self.receive() => {
    //             v
    //         },
    //
    //         v = tokio::time::sleep(Duration::from_millis(WS_TIMEOUT)) => {
    //             Err(OutgoingMessage::Timeout.into())
    //         },
    //     }
    // }
}
