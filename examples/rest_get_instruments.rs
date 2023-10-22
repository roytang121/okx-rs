use okx_rs::api::v5::model::InstrumentType::Spot;
use okx_rs::api::v5::testkit::with_public_client;
use okx_rs::api::{options::Options, v5::GetInstruments, Rest};

#[tokio::main]
async fn main() {
    with_public_client(|client| async move {
        let response = client
            .request(GetInstruments {
                inst_type: Spot,
                uly: None,
                inst_family: None,
                inst_id: None,
            })
            .await
            .unwrap();
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    })
    .await;
}
