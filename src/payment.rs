use serde::Serialize;

use crate::{
    constants::payment,
    models::{order::PaymentStatus, OrderNumber},
};

#[derive(Serialize)]
struct TossPaymentConfirmationRequest {
    #[serde(rename = "paymentKey")]
    payment_key: String,
    #[serde(rename = "orderId")]
    order_number: String,
    amount: i32,
}

pub struct ConfirmPaymentParams {
    pub secret_key: String,
    pub order_number: OrderNumber,
    pub payment_key: String,
    pub amount: i32,
}

pub async fn confirm_payment_with_toss(params: ConfirmPaymentParams) -> PaymentStatus {
    let confirm_request = TossPaymentConfirmationRequest {
        payment_key: params.payment_key,
        order_number: params.order_number.to_string(),
        amount: params.amount,
    };

    let response = reqwest::Client::new()
        .post(payment::TOSS_API_CONFIRM_URL)
        .basic_auth(&params.secret_key, Some(""))
        .json(&confirm_request)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => PaymentStatus::Paid,
        Ok(resp) => {
            let error_body = match resp.text().await {
                Ok(body) => body,
                Err(e) => {
                    tracing::error!("Failed to decode Toss API error response: {}", e);
                    "Failed to decode response".to_string()
                }
            };
            tracing::error!("Toss payment confirmation failed: {}", error_body);
            PaymentStatus::Failed
        }
        Err(e) => {
            tracing::error!("Failed to call Toss API: {}", e);
            PaymentStatus::Failed
        }
    }
}
