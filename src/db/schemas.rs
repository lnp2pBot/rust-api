use mongodb::bson::{oid::ObjectId, serde_helpers::bson_datetime_as_rfc3339_string, DateTime};
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
    _id: ObjectId,
    name: String,
    creator_id: String,
    order_channels: Vec<OrderChannel>,
    group: String,
    fee: f32,
    earnings: Option<f32>,
    orders_to_redeem: Option<f32>,
    dispute_channel: String,
    solvers: Vec<UsernameId>,
    banned_users: Option<Vec<UsernameId>>,
    public: bool,
    currencies: Vec<String>,
    #[serde(serialize_with = "bson_datetime_as_rfc3339_string::serialize")]
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    _id: ObjectId,
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
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DisputeStatus {
    #[serde(rename = "WAITING_FOR_SOLVER")]
    WaitingForSolver,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "SETTLED")]
    Settled,
    #[serde(rename = "SELLER_REFUNDED")]
    SellerRefunded,
    #[serde(rename = "RELEASED")]
    Released,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dispute {
    initiator: String,
    seller_id: Option<String>,
    buyer_id: Option<String>,
    status: DisputeStatus,
    community_id: Option<String>,
    order_id: String,
    solver_id: Option<String>,
    created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingPayment {
    description: String,
    amount: f32,
    attempts: f32,
    paid: bool,
    is_invoice_expired: bool,
    payment_request: String,
    hash: Option<String>,
    created_at: DateTime,
    paid_at: Option<DateTime>,
    user_id: String,
    order_id: Option<String>,
    community_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    description: String,
    amount: f32,
    fee: f32,
    bot_fee: f32,
    community_fee: f32,
    routing_fee: f32,
    hash: String,
    secret: String,
    creator_id: String,
    seller_id: String,
    buyer_id: String,
    buyer_invoice: String,
    buyer_dispute: bool,
    seller_dispute: bool,
    buyer_cooperativecancel: bool,
    seller_cooperativecancel: bool,
    canceled_by: Option<String>,
    status: OrderStatus,
    #[serde(rename = "type")]
    kind: String,
    fiat_amount: f32,
    fiat_code: String,
    payment_method: String,
    created_at: DateTime,
    invoice_held_at: Option<DateTime>,
    taken_at: Option<DateTime>,
    tg_chat_id: Option<String>,
    tg_order_message: Option<String>,
    tg_channel_message1: String,
    range_parent_id: Option<String>,
    price_from_api: bool,
    price_margin: f32,
    calculated: bool,
    admin_warned: bool,
    paid_hold_buyer_invoice_updated: bool,
    community_id: String,
}
