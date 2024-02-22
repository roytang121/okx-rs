use okx_rs::api::v5::testkit::with_env_private_client;
use okx_rs::api::v5::GetDepositHistory;

#[tokio::main]
async fn main() {
    with_env_private_client(|client| async move {
        let response = client.request(GetDepositHistory::default()).await.unwrap();
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    })
    .await;
}
