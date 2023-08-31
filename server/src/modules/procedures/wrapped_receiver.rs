use tokio::sync::mpsc::Receiver;

pub struct WrappedReceiver<T>(pub Receiver<T>);

impl<T> WrappedReceiver<T> {
    pub async fn receive_with_timeout(&mut self) {
        tokio::select! {
            v = self.0.receive() => {
                v
            },

            v = tokio::time::sleep(Duration::from_millis(WS_TIMEOUT)) => {
                Err(OutgoingMessage::Timeout.into())
            },
        }
    }
}
