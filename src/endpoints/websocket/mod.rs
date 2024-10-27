mod message;

use actix_web::{get, rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use data_access::DatabaseConnection;
use data_models::{MessageData, WebSocketMessage};
use message::handle_message;

use crate::jwt_handler::Claims;

#[get("/")]
pub async fn echo(
    req: HttpRequest,
    stream: web::Payload,
    claims: web::ReqData<Claims>,
    repository: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;
    println!("WebSocket connection initiated");

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(Ok(msg)) = stream.recv().await {
            let mut session = session.clone();
            match msg {
                AggregatedMessage::Text(text) => {
                    handle_json_request(&text, claims.id, &mut session, &repository).await
                }
                AggregatedMessage::Ping(msg) => {
                    println!("Ping received");
                    session.pong(&msg).await.unwrap()
                }
                AggregatedMessage::Close(msg) => {
                    println!("Session Closed");
                    session.close(msg).await.unwrap()
                }
                _ => {}
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

    let output = match message.data {
        MessageData::Channel(t) => Ok(println!("Channel: {:?}", t)),
        MessageData::Guild(t) => Ok(println!("Guild: {:?}", t)),
        MessageData::Message(m) => handle_message(&m, user_id, repository).await,
        MessageData::Ack(s) => Ok(println!("Ack {}", s)),
    };

    if output.is_err() {
        let response = WebSocketMessage {
            data: MessageData::Ack(String::from(
                "An error occurred while processing your request",
            )),
        };

        session
            .text(serde_json::to_string(&response).expect("Unable to stringify message"))
            .await
            .expect("Unable to send message to client");

        return;
    } else {
        let response = WebSocketMessage {
            data: MessageData::Ack(String::from("ack")),
        };

        session
            .text(serde_json::to_string(&response).expect("Unable to stringify message"))
            .await
            .expect("unable to send message to client");
    }
}
