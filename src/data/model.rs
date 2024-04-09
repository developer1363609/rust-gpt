use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub id:i64,
    pub email:String,
    pub password:String,
    pub created_at:DateTime<Utc>,
    openai_api_key:
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Chat{
    pub id:i64,
    pub name:String,
    pub user_id:i64
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ChatMessagePair{
    pub id:i64,
    pub model:String,
    pub message_block_id:i64,
    pub chat_id:i64,
    pub human_message:String,
    pub ai_message:Option<String>,
    pub block_rank:i64,
    pub block_size:i64
}