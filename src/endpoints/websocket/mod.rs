mod message;

use actix_web::{get, rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, CloseReason};
use data_access::DatabaseConnection;
use data_models::{MessageData, WebSocketMessage};
use message::handle_message;

#[get("/")]
pub async fn gateway(
    req: HttpRequest,
    stream: web::Payload,
    repository: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = match actix_ws::handle(&req, stream) {
        Ok(res) => {
            res
        }
        Err(e) => {
            return Err(Error::from(e));
        }
    };
    println!("WebSocket connection initiated");

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        let hello_message = stream
            .recv()
            .await
            .and_then(|result| result.ok())
            .and_then(|msg| match msg {
                AggregatedMessage::Text(addr) => Some(addr),
                _ => None,
            });
        if hello_message.is_none()
            || match handle_hello_request(&hello_message.unwrap()) {
                Ok(_) => false,
                Err(e) => {
                    eprintln!("Error handling hello request: {:?}", e);
                    true
                }
            }
        {
            let _ = session
                .close(Some(CloseReason {
                    code: actix_ws::CloseCode::Invalid,
                    description: Some(String::from("Invalid hello message")),
                }))
                .await;
            return;
        }
        
        while let Some(Ok(msg)) = stream.recv().await {
            let mut session = session.clone();
            match msg {
                AggregatedMessage::Text(text) => {
                    handle_json_request(&text, 0, &mut session, &repository).await
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

fn handle_hello_request(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    match serde_json::from_str::<WebSocketMessage>(message)?.data {
        MessageData::Hello(h) => {
            println!("{:?}", h);
            Ok(())
        }
        MessageData::Channel(_) => Err(Box::from(
            "Received create channel request but expected hello.",
        )),
        MessageData::Guild(_) => Err(Box::from(
            "Received create guild request but expected hello.",
        )),
        MessageData::Message(_) => Err(Box::from(
            "Received create message request but expected hello.",
        )),
        MessageData::Ack(_) => Err(Box::from("Received ack but expected hello.")),
    }
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
        MessageData::Hello(_) => Err(Box::from("You cannot send multiple hellos per session.")),
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
