use tokio::sync::oneshot::Reciever;

pub struct ShutdownHandler {
  rx : Reciever
}

impl ShutdownHandler {
  pub new (rx: Reciever) -> Self {
    ShutdownHandler {
      rx,
    }
  }
}