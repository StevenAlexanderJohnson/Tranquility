use data_access::{DatabaseConnection, MessageResponse};
use data_models::CreateMessageRequest;

pub async fn handle_message(
    message: &CreateMessageRequest,
    user_id: i32,
    repository: &DatabaseConnection,
) -> Result<MessageResponse, Box<dyn std::error::Error>> {
    let message = repository.create_message(message, user_id).await?;
    Ok(message)
}
