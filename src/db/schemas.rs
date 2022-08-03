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
    id: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderChannel {
    name: String,
    #[serde(rename = "type")]
    kind: ChannelType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Community {
    #[serde(rename = "_id", serialize_with = "serialize_oid_as_string")]
    id: ObjectId,
    name: String,
    order_channels: Vec<OrderChannel>,
    group: String,
    fee: f32,
    earnings: Option<f32>,
    orders_to_redeem: Option<f32>,
    dispute_channel: String,
    solvers: Vec<UsernameId>,
    public: bool,
    currencies: Vec<String>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", serialize_with = "serialize_oid_as_string")]
    id: ObjectId,
    tg_id: String,
    username: String,
    lang: String,
    trades_completed: f32,
    total_reviews: f32,
    last_rating: f32,
    total_rating: f32,
    volume_traded: f32,
    admin: bool,
    banned: bool,
    show_username: bool,
    show_volume_traded: bool,
    lightning_address: Option<String>,
    disputes: Option<f32>,
    default_community_id: Option<String>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(rename = "_id", serialize_with = "serialize_oid_as_string")]
    id: ObjectId,
    description: String,
    amount: f32,
    fee: f32,
    bot_fee: Option<f32>,
    community_fee: Option<f32>,
    status: OrderStatus,
    #[serde(rename = "type")]
    direction: String,
    fiat_amount: Option<f32>,
    fiat_code: String,
    payment_method: String,
    taken_at: Option<DateTime>,
    tg_chat_id: Option<String>,
    tg_order_message: Option<String>,
    tg_channel_message1: String,
    price_from_api: Option<bool>,
    price_margin: Option<f32>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    created_at: DateTime,
}

// Requests to be used by routes
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderRequest {
    pub id: Option<String>,
    pub status: Option<OrderStatus>,
    pub direction: Option<String>,
    pub fiat_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityRequest {
    pub id: Option<String>,
    pub currency: Option<String>,
}
