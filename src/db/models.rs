use mongodb::bson;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "mixed")]
    Mixed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsernameId {
    id: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderChannel {
    name: String,
    #[serde(rename = "type")]
    kind: ChannelType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    _id: bson::oid::ObjectId,
    name: String,
    creator_id: String,
    order_channels: Vec<OrderChannel>,
    group: String,
    fee: f32,
    earnings: Option<f32>,
    orders_to_redeem: Option<f32>,
    dispute_channel: String,
    solvers: Vec<UsernameId>,
    banned_users: Vec<UsernameId>,
    public: bool,
    currencies: Vec<String>,
    created_at: DateTime,
}
