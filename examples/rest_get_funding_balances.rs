use okx_rs::api::v5::funding_account::GetFundingBalances;
use okx_rs::api::v5::testkit::with_env_private_client;

#[tokio::main]
async fn main() {
    with_env_private_client(|client| async move {
        let response = client
            .request(GetFundingBalances {
                ..Default::default()
            })
            .await
            .unwrap();
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    })
    .await;
}
