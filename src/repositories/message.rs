use futures::stream::{StreamExt};
use mongodb::{Collection, Client, bson::{oid::ObjectId, doc, DateTime}, error::Result, options::FindOptions};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MessageModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub text: String,
    pub post_moment: DateTime,
}

#[derive(Debug, Clone)]
pub struct MessageRepo {
    inner: Collection<MessageModel>,
}

impl MessageRepo {
    const DATABASE: &'static str = "Poster";
    const COLLECTION: &'static str = "Messages";

    pub async fn new(uri: String) -> Self {
        let client = log_error!(Client::with_uri_str(uri).await);
        let db = client.database(Self::DATABASE);
        let inner: Collection<MessageModel> = db.collection(Self::COLLECTION);
        Self { inner }
    }

    pub async fn create_message(&self, mut new_msg: MessageModel) -> Result<()> {
        new_msg.id = None;
        self
            .inner
            .insert_one(new_msg, None)
            .await
            .ok()
            .expect("Error creating message");
        Ok(())
    }

    pub async fn get_messages(&self, skip: u64, limit: u64) -> Result<Vec<MessageModel>> {
        let find_options = FindOptions::builder()
            .sort(doc!{ "post_moment" : -1 })
            .skip(skip).limit(limit as i64)
            .build();
        
        let messages: Vec<Result<MessageModel>> = self
            .inner
            .find(None, find_options)
            .await?
            .collect()
            .await;
        
        messages.into_iter().collect()
    }
}
