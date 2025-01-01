use data_models::MessageResponse;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[sqlx(default)]
pub struct Message {
    pub id: Option<i32>,
    pub channel_id: Option<i32>,
    pub author: Option<String>,
    pub author_id: Option<i32>,
    pub content: Option<String>,
    pub created_date: Option<DateTime<Utc>>,
    pub updated_date: Option<DateTime<Utc>>,
}

impl TryFrom<Message> for MessageResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or::<Self::Error>(
                "ID was not provided while casting Message to MessageResponse".into(),
            )?,
            author: value.author.ok_or::<Self::Error>(
                "Author was not provided while casting Message to MessageREsponse".into()
            )?,
            author_id: value.author_id.ok_or::<Self::Error>(
                "Author ID was not provided while casting Message to MessageResponse".into(),
            )?,
            content: value.content.ok_or::<Self::Error>(
                "Author ID was not provided while casting Message to MessageResponse".into(),
            )?,
            attachments: vec![],
            created_date: value.created_date.ok_or::<Self::Error>(
                "Author ID was not provided while casting Message to MessageResponse".into(),
            )?,
            updated_date: value.updated_date.ok_or::<Self::Error>(
                "Author ID was not provided while casting Message to MessageResponse".into(),
            )?,
        })
    }
}
