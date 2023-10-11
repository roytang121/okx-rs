use super::*;
use serde_json::json;

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Channel {
    OrderBook { channel: String, inst_id: String },
}

impl WebsocketConn for Channel {
    fn subscribe(&self, client: &mut dyn WebsocketClient) -> Result<()> {
        match self {
            Channel::OrderBook { channel, inst_id } => {
                client.send(
                    &json!({
                        "op": "subscribe",
                        "args": [
                            {
                                "channel": channel,
                                "instId": inst_id,
                            }
                        ]
                    })
                    .to_string(),
                )?;
            }
        }
        Ok(())
    }

    fn unsubscribe(&self, client: &mut dyn WebsocketClient) -> Result<()> {
        todo!()
    }
}
