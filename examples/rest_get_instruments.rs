use okx_rs::api::v5::model::InstrumentType::Spot;
use okx_rs::api::v5::GetInstruments;
use okx_rs::api::{blocking, Options, Production};

fn main() {
    env_logger::try_init().unwrap();

    let client = blocking::Rest::new(Options::new(Production));
    let response = client
        .request(GetInstruments {
            inst_type: Spot,
            uly: None,
            inst_family: None,
            inst_id: None,
        })
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}
