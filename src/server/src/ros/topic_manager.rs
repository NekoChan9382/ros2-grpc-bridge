use super::json_codec;
use rclrs::{DynamicPublisher, DynamicSubscription, MessageTypeName, Node};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::broadcast;

const BROADCAST_CAPACITY: usize = 32;

#[derive(Clone)]
pub struct BridgeMessage {
    pub topic: String,
    pub msg_type: String,
    pub json: Vec<u8>,
}

pub struct TopicState {
    msg_type: MessageTypeName,
    publisher: Mutex<Option<DynamicPublisher>>,
    subscription: Mutex<Option<DynamicSubscription>>,
    tx: broadcast::Sender<Arc<BridgeMessage>>,
}

pub struct TopicManager {
    node: Node,
    topics: RwLock<HashMap<String, Arc<TopicState>>>,
}

impl TopicManager {
    pub fn new(node: Node) -> Self {
        Self {
            node,
            topics: RwLock::new(HashMap::new()),
        }
    }

    pub fn publish(&self, topic: &str, msg_type: &str, json: &[u8]) -> Result<(), String> {
        let state = self.state(topic, msg_type)?;
        let message = json_codec::from_json(state.msg_type.clone(), json)?;
        let mut publisher = state
            .publisher
            .lock()
            .map_err(|_| "publisher lock poisoned")?;
        if publisher.is_none() {
            *publisher = Some(
                self.node
                    .create_dynamic_publisher(state.msg_type.clone(), topic)
                    .map_err(|error| error.to_string())?,
            );
        }
        publisher
            .as_ref()
            .expect("publisher initialized")
            .publish(message)
            .map_err(|error| error.to_string())
    }

    pub fn subscribe(
        &self,
        topic: &str,
        msg_type: &str,
    ) -> Result<broadcast::Receiver<Arc<BridgeMessage>>, String> {
        let state = self.state(topic, msg_type)?;
        self.ensure_subscription(topic, &state)?;
        Ok(state.tx.subscribe())
    }

    fn state(&self, topic: &str, msg_type: &str) -> Result<Arc<TopicState>, String> {
        let message_type =
            MessageTypeName::try_from(msg_type).map_err(|error| error.to_string())?;
        let mut topics = self
            .topics
            .write()
            .map_err(|_| "topic manager lock poisoned")?;
        if let Some(state) = topics.get(topic) {
            if state.msg_type != message_type {
                return Err(format!(
                    "topic '{topic}' is already registered as {}",
                    state.msg_type
                ));
            }
            return Ok(Arc::clone(state));
        }
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        let state = Arc::new(TopicState {
            msg_type: message_type,
            publisher: Mutex::new(None),
            subscription: Mutex::new(None),
            tx,
        });
        topics.insert(topic.to_owned(), Arc::clone(&state));
        Ok(state)
    }

    fn ensure_subscription(&self, topic: &str, state: &Arc<TopicState>) -> Result<(), String> {
        let mut subscription = state
            .subscription
            .lock()
            .map_err(|_| "subscription lock poisoned")?;
        if subscription.is_some() {
            return Ok(());
        }
        let tx = state.tx.clone();
        let topic_name = topic.to_owned();
        let type_name = state.msg_type.to_string();
        *subscription = Some(
            self.node
                .create_dynamic_subscription(
                    state.msg_type.clone(),
                    topic,
                    move |message, _info| match json_codec::to_json(&message) {
                        Ok(json) => {
                            let _ = tx.send(Arc::new(BridgeMessage {
                                topic: topic_name.clone(),
                                msg_type: type_name.clone(),
                                json,
                            }));
                        }
                        Err(error) => eprintln!("failed to convert ROS message to JSON: {error}"),
                    },
                )
                .map_err(|error| error.to_string())?,
        );
        Ok(())
    }
}
