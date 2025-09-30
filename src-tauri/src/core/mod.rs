use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub chain: String,
    pub hash: String,
    pub from_address: String,
    pub to_address: Option<String>,
    pub value: Decimal,
    pub token_symbol: String,
    pub token_decimals: i32,
    pub timestamp: DateTime<Utc>,
    pub block_number: i64,
    pub transaction_type: String,
    pub status: String,
    pub fee: Option<Decimal>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub decimals: u8,
    pub chain: String,
    pub contract_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub address: String,
    pub chain: String,
    pub nickname: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub account_id: Uuid,
    pub token_symbol: String,
    pub chain: String,
    pub amount: Decimal,
    pub updated_at: DateTime<Utc>,
}