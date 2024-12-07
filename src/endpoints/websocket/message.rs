use data_access::DatabaseConnection;
use data_models::CreateMessageRequest;

pub async fn handle_message(
    message: &CreateMessageRequest,
    user_id: i32,
    repository: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", message);
    repository.create_message(message, user_id).await?;
    Ok(())
}
