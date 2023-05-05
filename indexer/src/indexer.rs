use bombastic_event_bus::{Event, EventBus};
use bombastic_index::Index;
use bombastic_storage::Storage;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize, Debug)]
pub struct StorageEvent {
    #[serde(rename = "EventName")]
    event_name: String,
    #[serde(rename = "Key")]
    key: String,
}

const PUT_EVENT: &str = "s3:ObjectCreated:Put";

pub async fn run<E: EventBus>(mut index: Index, storage: Storage, event_bus: E) -> Result<(), anyhow::Error> {
    loop {
        match event_bus.poll().await {
            Ok(event) => loop {
                if let Some(payload) = event.payload() {
                    if let Ok(data) = serde_json::from_slice::<StorageEvent>(payload) {
                        tracing::info!("Got payload from event: {:?}", data);
                        if data.event_name == PUT_EVENT {
                            if let Some(key) = storage.extract_key(&data.key) {
                                match storage.get(key).await {
                                    Ok(data) => {
                                        // TODO: Handle SPDX and move this out to a separate module, and use serde instead of raw JSON.
                                        let j = serde_json::from_slice::<serde_json::Value>(&data).unwrap();
                                        let purl = j["metadata"]["component"]["purl"].as_str().unwrap();
                                        let mut hasher = Sha256::new();
                                        hasher.update(&data);
                                        let hash = hasher.finalize();
                                        match index.insert(purl, &format!("{:x}", hash), key).await {
                                            Ok(_) => tracing::info!("Inserted entry into index"),
                                            Err(e) => tracing::warn!("Error inserting entry into index: {:?}", e),
                                        }
                                    }
                                    Err(e) => {}
                                }
                            } else {
                                tracing::warn!("Error extracting key from event: {:?}", data)
                            };
                        }
                    }
                }
                match event.commit() {
                    Ok(_) => {
                        tracing::info!("Event committed successfully");
                        break;
                    }
                    Err(e) => {
                        tracing::warn!("Error committing event: {:?}", e)
                    }
                }
            },
            Err(e) => {
                tracing::warn!("Error polling for event: {:?}", e);
            }
        }
    }
    Ok(())
}