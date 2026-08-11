use rclrs::{Executor, Node, Publisher, RclrsError, Subscription};
use sensor_msgs::msg::Joy;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct JoyData {
    pub axes: Vec<f32>,
    pub buttons: Vec<i32>,
}

pub struct JoyNode {
    node: Arc<Node>,
    publisher: Arc<Publisher<Joy>>,
    _subscription: Arc<Subscription<Joy>>,
}

impl JoyNode {
    pub fn new(
        executor: &Executor,
        joy_tx: broadcast::Sender<JoyData>,
    ) -> Result<Self, RclrsError> {
        let node = executor.create_node("grpc_bridge")?;
        let publisher = node.create_publisher::<Joy>("/joy")?;

        let tx = joy_tx.clone();
        let subscription = node.create_subscription("/test", move |msg: Joy| {
            let data = JoyData {
                axes: msg.axes.clone(),
                buttons: msg.buttons.clone(),
            };
            println!("data came {}", msg.axes[0]);
            let _ = tx.send(data);
        })?;
        Ok(Self {
            node: Arc::new(node),
            publisher: Arc::new(publisher),
            _subscription: Arc::new(subscription),
        })
    }
    pub fn publish(&self, data: JoyData) -> Result<(), RclrsError> {
        let mut msg = Joy::default();
        msg.axes = data.axes;
        msg.buttons = data.buttons;
        self.publisher.publish(msg)
    }
}
