use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use std::sync::Arc;
use tokio::sync::Mutex;

mod models;
use models::{Transaction, TransactionType};

#[derive(Clone)]
struct AppState {
    transactions: Arc<Mutex<Vec<Transaction>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        transactions: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route(
            "/transactions",
            get(get_transactions).post(create_transaction),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_transactions(
    State(state): State<AppState>,
) -> Json<Vec<Transaction>> {
    let txs = state.transactions.lock().await;
    Json(txs.clone())
}

#[derive(serde::Deserialize)]
struct CreateTransaction {
    title: String,
    amount: f64,
    transaction_type: TransactionType,
}

async fn create_transaction(
    State(state): State<AppState>, Json(payload): Json<CreateTransaction>,
) -> Json<Transaction> {
    let mut txs = state.transactions.lock().await;
    let new_tx = Transaction {
        id: uuid::Uuid::new_v4(),
        title: payload.title,
        amount: payload.amount,
        transaction_type: payload.transaction_type,
        created_at: chrono::Utc::now(),
    };
    txs.push(new_tx.clone());
    Json(new_tx)
}
