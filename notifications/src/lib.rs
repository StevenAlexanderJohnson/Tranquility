use models::WebsocketResponseData;
use std::collections::HashMap;
use tokio::{
    io,
    sync::{mpsc, oneshot},
};

pub mod models;

type ConnId = usize;

#[derive(Debug)]
enum Command {
    Connect {
        user_id: ConnId,
        conn_tx: mpsc::UnboundedSender<WebsocketResponseData>,
        res_tx: oneshot::Sender<()>,
    },

    Disconnect {
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },

    Message {
        msg: WebsocketResponseData,
        res_tx: oneshot::Sender<()>,
    },
}

#[derive(Debug)]
pub struct WebsocketServer {
    sessions: HashMap<ConnId, mpsc::UnboundedSender<WebsocketResponseData>>,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl WebsocketServer {
    pub fn new() -> (Self, WebsocketServerHandler) {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<Command>();

        (
            Self {
                sessions: HashMap::new(),
                cmd_rx,
            },
            WebsocketServerHandler { cmd_tx },
        )
    }

    async fn send_system_message(&self, msg: WebsocketResponseData) {
        for (_, tx) in &self.sessions {
            let msg = msg.clone();
            let _ = tx.send(msg).unwrap();
        }
    }

    async fn list_connected(&self) -> Vec<ConnId> {
        return self.sessions.keys().cloned().collect();
    }

    async fn connect(&mut self, user_id: ConnId, tx: mpsc::UnboundedSender<WebsocketResponseData>) {
        log::info!("Someone joined");

        self.sessions.insert(user_id, tx);

        self.send_system_message(WebsocketResponseData::Ack("Someone has joined".into()))
            .await;
    }

    async fn disconnect(&mut self, conn_id: ConnId, tx: oneshot::Sender<()>) {
        log::info!("Someone disconnected");

        if self.sessions.remove(&conn_id).is_some() {
            self.send_system_message(WebsocketResponseData::Ack("User Removed".into()))
                .await;
        }

        let _ = tx.send(()).unwrap();
    }

    pub async fn run(mut self) -> io::Result<()> {
        log::info!("Notification Server is running.");
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect {
                    user_id,
                    conn_tx,
                    res_tx,
                } => {
                    let conn_id = self.connect(user_id, conn_tx).await;
                    let _ = res_tx.send(conn_id).unwrap();
                    // let conn_id = self.sessions.insert(0, conn_tx);
                }
                Command::Disconnect { conn, res_tx } => {
                    self.disconnect(conn, res_tx).await;
                }
                Command::Message { msg, res_tx } => {
                    self.send_system_message(msg).await;
                    let _ = res_tx.send(()).unwrap();
                }
            }
        }
        Ok(())
    }
}

/// # WebsocketServerHandler
///
/// This struct is passed into actix as a Data dependency.
/// It is initialized when `WebsocketServer` gets initialized.
///
/// ## Properties
///
/// * `cmd_tx`: This is a channel that is able to send `Command` values to the websocket server.
#[derive(Debug, Clone)]
pub struct WebsocketServerHandler {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl WebsocketServerHandler {
    pub async fn connect(
        &self,
        user_id: ConnId,
        conn_tx: mpsc::UnboundedSender<WebsocketResponseData>,
    ) {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::Connect {
                user_id,
                conn_tx: conn_tx,
                res_tx: res_tx,
            })
            .unwrap();

        res_rx.await.unwrap()
    }

    pub async fn disconnect(&self, connection_id: ConnId) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::Disconnect {
                conn: connection_id,
                res_tx,
            })
            .unwrap();

        res_rx.await.unwrap()
    }

    pub async fn send_message(&self, msg: WebsocketResponseData) {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::Message {
                msg: msg,
                res_tx: res_tx,
            })
            .unwrap();

        res_rx.await.unwrap();
    }
}
