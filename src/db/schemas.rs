use mongodb::bson::{oid::ObjectId, serde_helpers::bson_datetime_as_rfc3339_string, DateTime};
use rocket::serde::{Deserialize, Serialize};
use serde::Serializer;

pub fn serialize_oid_as_string<S>(oid: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(oid.to_string().as_str())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "mixed")]
    Mixed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsernameId {
    username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderChannel {
    name: String,
    #[serde(rename = "type")]
    channel_type: ChannelType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Community {
    #[serde(serialize_with = "serialize_oid_as_string")]
    _id: ObjectId,
    name: String,
    order_channels: Vec<OrderChannel>,
    group: String,
    fee: f32,
    earnings: Option<f32>,
    orders_to_redeem: Option<f32>,
    dispute_channel: String,
    creator_id: String,
    solvers: Vec<UsernameId>,
    public: bool,
    currencies: Vec<String>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(serialize_with = "serialize_oid_as_string")]
    _id: ObjectId,
    username: String,
    trades_completed: f32,
    total_reviews: f32,
    total_rating: f32,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OrderStatus {
    #[serde(rename = "WAITING_PAYMENT")]
    WaitingPayment,
    #[serde(rename = "WAITING_BUYER_INVOICE")]
    WaitingBuyerInvoice,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "FIAT_SENT")]
    FiatSent,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "DISPUTE")]
    Dispute,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "PAID_HOLD_INVOICE")]
    PaidHoldInvoice,
    #[serde(rename = "CANCELED_BY_ADMIN")]
    CanceledByAdmin,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "COMPLETED_BY_ADMIN")]
    CompletedByAdmin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(serialize_with = "serialize_oid_as_string")]
    pub _id: ObjectId,
    pub description: String,
    pub amount: f32,
    pub fee: f32,
    pub bot_fee: Option<f32>,
    pub community_fee: Option<f32>,
    pub status: OrderStatus,
    #[serde(rename = "type")]
    pub direction: String,
    pub fiat_amount: Option<f32>,
    pub min_amount: Option<f32>,
    pub max_amount: Option<f32>,
    pub fiat_code: String,
    pub payment_method: String,
    pub taken_at: Option<DateTime>,
    pub tg_chat_id: Option<String>,
    pub tg_order_message: Option<String>,
    pub tg_channel_message1: Option<String>,
    pub price_from_api: Option<bool>,
    pub price_margin: Option<f32>,
    pub community_id: Option<String>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    pub created_at: DateTime,
}

// Requests to be used by routes
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderRequest {
    pub _id: Option<String>,
    pub status: Option<String>,
    pub direction: Option<String>,
    pub currency: Option<String>,
    pub community_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityRequest {
    pub _id: Option<String>,
    pub currency: Option<String>,
}
