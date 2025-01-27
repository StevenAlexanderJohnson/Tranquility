use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use actix_web::{error::ErrorUnauthorized, get, rt, web, Error, HttpRequest, HttpResponse};

use actix_ws::{AggregatedMessage, CloseCode, CloseReason};
use data_access::DatabaseConnection;
use log::{error, info};
use message::handle_message;
use notifications::{
    models::{WebsocketMessage, WebsocketMessageData, WebsocketResponseData},
    WebsocketServerHandler,
};
use tokio::{select, sync::mpsc, time::interval};

mod message;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
enum DisconnectReason {
    Timeout,
    Disconnect,
    Error,
}

#[get("/{id}/{token}")]
pub async fn gateway(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(i32, String)>,
    repository: web::Data<DatabaseConnection>,
    websocket_server: web::Data<WebsocketServerHandler>,
) -> Result<HttpResponse, Error> {
    // Handle the handshake
    let (res, session, stream) = match actix_ws::handle(&req, stream) {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    info!("WebSocket connection initiated");

    // Login with path variables
    let user = match repository.websocket_login(path.0, &path.1).await {
        Ok(user) => user,
        Err(e) => {
            error!("Error while logging in in websocket: {:?}", e);
            let _ = session
                .close(Some(CloseReason {
                    code: actix_ws::CloseCode::Invalid,
                    description: Some(String::from("Unable to authenticate user.")),
                }))
                .await;
            return Err(ErrorUnauthorized("Invalid login"));
        }
    };

    let mut stream = stream
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        let websocket_server = websocket_server.clone().into_inner();
        let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

        let mut last_heartbeat = Instant::now();
        let mut interval = interval(HEARTBEAT_INTERVAL);

        websocket_server.connect(user.id as usize, conn_tx).await;

        let close_reason: DisconnectReason = loop {
            select! {
                // Handles messages coming in over the websocket from the user.
                stream_message = stream.recv() => {
                    match stream_message {
                        Some(Ok(msg)) => {
                            let mut session = session.clone();
                            match msg {
                                AggregatedMessage::Text(text) => {
                                    let message: WebsocketMessage =
                                        serde_json::from_str(&text).expect("Unable to deserialize message");

                                    if let WebsocketMessage::Ping(_) = message {
                                        last_heartbeat = Instant::now();
                                        session.pong(b"").await.unwrap();
                                        continue;
                                    }

                                    handle_json_request(
                                        &websocket_server,
                                        &message,
                                        user.id,
                                        &mut session,
                                        &repository,
                                    )
                                    .await
                                }
                                AggregatedMessage::Ping(msg) => {
                                    last_heartbeat = Instant::now();
                                    info!("Ping received, {}", user.id);
                                    session.pong(&msg).await.unwrap()
                                }
                                AggregatedMessage::Close(msg) => {
                                    info!("Session Closed");
                                    session.close(msg).await.unwrap();
                                    break DisconnectReason::Disconnect;
                                }
                                _ => {
                                    error!("How did you get here")
                                }
                            }
                        },
                        Some(Err(e)) => {
                            log::error!("An error occurred while receiving stream message: {} -> {:?}", user.id, e);
                            break DisconnectReason::Error;
                        },
                        None => {
                            log::error!("None was sent over the stream: {}", user.id);
                            break DisconnectReason::Error;
                        }
                    }
                },
                // Handles messages coming from the server to the websocket.
                msg = conn_rx.recv() => {
                    let mut session = session.clone();
                    session
                        .text(serde_json::to_string(&msg).expect("Unable to stringify message"))
                        .await
                    .expect("unable to send message to client");
                },
                _ = interval.tick() => {
                    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                        log::info!("Client {} has not sent a heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting...", user.id);
                        break DisconnectReason::Timeout;
                    }
                }
            };
        };

        websocket_server.disconnect(user.id as usize).await;
        let _ = session
            .close(Some(CloseReason {
                code: CloseCode::Protocol,
                description: Some(String::from(
                    "Connection has been closed due to lack of heartbeat.",
                )),
            }))
            .await;

        log::info!(
            "Websocket loop closed: UserId -> {}, Reason -> {:?}",
            user.id,
            close_reason
        );
    });

    Ok(res)
}

async fn handle_json_request(
    chat_server: &Arc<WebsocketServerHandler>,
    message: &WebsocketMessage,
    user_id: i32,
    session: &mut actix_ws::Session,
    repository: &DatabaseConnection,
) {
    let message = match message {
        WebsocketMessage::Ping(_) => {
            log::warn!("An ping message made it to handle_json_request");
            return;
        }
        WebsocketMessage::Data(d) => d,
    };

    let output: Result<WebsocketResponseData, Box<dyn std::error::Error>> = match message {
        WebsocketMessageData::Channel(_) => Ok(WebsocketResponseData::Ack("Ack".to_string())),
        WebsocketMessageData::Guild(_) => Ok(WebsocketResponseData::Ack("Ack".to_string())),
        WebsocketMessageData::Message(m) => handle_message(&m, user_id, repository)
            .await
            .map(WebsocketResponseData::Message),
        WebsocketMessageData::Ack(_) => Ok(WebsocketResponseData::Ack("Ack".to_string())),
        _ => return,
    };

    match output {
        Ok(x) => {
            chat_server.send_message(x).await;
        }
        Err(e) => {
            error!("{:?}", e);
            let response = WebsocketMessage::Data(WebsocketMessageData::Ack(String::from(
                "An error occurred while processing your request",
            )));

            session
                .text(serde_json::to_string(&response).expect("Unable to stringify message"))
                .await
                .expect("Unable to send message to client");
        }
    }
}
