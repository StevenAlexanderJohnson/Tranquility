use actix_web::{get, rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use data_models::{CreateGuildRequest, MessageData, WebSocketMessage};

#[get("/")]
pub async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
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
                AggregatedMessage::Text(text) => handle_json_request(&text, &mut session).await,
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

async fn handle_json_request(message: &str, session: &mut actix_ws::Session) {
    let message: WebSocketMessage =
        serde_json::from_str(message).expect("Unable to deserialize message");

    match message.data {
        Some(MessageData::Channel(t)) => println!("{:?}", t),
        Some(MessageData::Guild(t)) => println!("{:?}", t),
        None => println!("No Data"),
    }

    let response = WebSocketMessage {
        message_type: "Ack".into(),
        data: Some(MessageData::Guild(CreateGuildRequest {
            description: "Something".into(),
            name: "else".into()
        })),
    };
    session
        .text(serde_json::to_string(&response).expect("Unable to stringify message"))
        .await
        .expect("unable to send message to client");
}
