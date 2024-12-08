mod message;

use actix_web::{error::ErrorUnauthorized, get, rt, web, Error, HttpRequest, HttpResponse};

use actix_ws::{AggregatedMessage, CloseReason};
use data_access::DatabaseConnection;
use data_models::{WebsocketMessageData, WebSocketMessage};
use message::handle_message;

#[get("/{id}/{token}")]
pub async fn gateway(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(i32, String)>,
    repository: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Handle the handshake
    let (res, session, stream) = match actix_ws::handle(&req, stream) {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    println!("WebSocket connection initiated");

    // Login with path variables
    let user = match repository.websocket_login(path.0, &path.1).await {
        Ok(user) => user,
        Err(e) => {
            println!("Error while logging in in websocket: {:?}", e);
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
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(Ok(msg)) = stream.recv().await {
            let mut session = session.clone();
            match msg {
                AggregatedMessage::Text(text) => {
                    handle_json_request(&text, user.id, &mut session, &repository).await
                }
                AggregatedMessage::Ping(msg) => {
                    println!("Ping received");
                    session.pong(&msg).await.unwrap()
                }
                AggregatedMessage::Close(msg) => {
                    println!("Session Closed");
                    session.close(msg).await.unwrap()
                }
                _ => {
                    println!("How did you get here")
                }
            }
        }
    });

    Ok(res)
}

async fn handle_json_request(
    message: &str,
    user_id: i32,
    session: &mut actix_ws::Session,
    repository: &DatabaseConnection,
) {
    let message: WebSocketMessage =
        serde_json::from_str(message).expect("Unable to deserialize message");

    let output: Result<(), Box<dyn std::error::Error>> = match message.data {
        WebsocketMessageData::Channel(t) => Ok(println!("Channel: {:?}", t)),
        WebsocketMessageData::Guild(t) => Ok(println!("Guild: {:?}", t)),
        WebsocketMessageData::Message(m) => Ok(println!(
            "{:?}",
            handle_message(&m, user_id, repository).await
        )),
        WebsocketMessageData::Ack(s) => Ok(println!("Ack {}", s)),
    };

    if output.is_err() {
        let response = WebSocketMessage {
            data: WebsocketMessageData::Ack(String::from(
                "An error occurred while processing your request",
            )),
        };

        session
            .text(serde_json::to_string(&response).expect("Unable to stringify message"))
            .await
            .expect("Unable to send message to client");
    } else {
        let response = WebSocketMessage {
            data: WebsocketMessageData::Ack(String::from("ack")),
        };

        session
            .text(serde_json::to_string(&response).expect("Unable to stringify message"))
            .await
            .expect("unable to send message to client");
    }
}
