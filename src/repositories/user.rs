use futures::StreamExt;
use mongodb::{Collection, Client, bson::{oid::ObjectId, doc}, error::Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nickname: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserRepo {
    inner: Collection<UserModel>,
}

impl UserRepo {
    const DATABASE: &'static str = "Poster";
    const COLLECTION: &'static str = "Users";

    
    pub async fn new(uri: String) -> Self {
        let client = log_error!(Client::with_uri_str(uri).await);
        let db = client.database(Self::DATABASE);
        let inner: Collection<UserModel> = db.collection(Self::COLLECTION);
        Self { inner }
    }

    pub async fn create_user(&self, mut user: UserModel) -> Result<()> {
        user.id = None;
        self
            .inner
            .insert_one(user, None)
            .await?;
            
        Ok(())
    }

    pub async fn get_user(&self, nickname: String) -> Result<Option<UserModel>> {
        let user = self
            .inner
            .find(doc!{ "nickname" : nickname }, None)
            .await?
            .next()
            .await;

        user.map_or(Ok(None), |v| v.map(Some))
    }
}
