use crate::robot::{PublishResponse, SubscribeRequest, TopicMessage, ros_bridge_server::RosBridge};
use crate::ros::topic_manager::TopicManager;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::wrappers::{BroadcastStream, errors::BroadcastStreamRecvError};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};

pub struct BridgeServer {
    pub topics: Arc<TopicManager>,
}

#[tonic::async_trait]
impl RosBridge for BridgeServer {
    async fn publish(
        &self,
        request: Request<TopicMessage>,
    ) -> Result<Response<PublishResponse>, Status> {
        let request = request.into_inner();
        self.topics
            .publish(&request.topic, &request.r#type, &request.json)
            .map_err(Status::invalid_argument)?;
        Ok(Response::new(PublishResponse {
            success: true,
            msg: "published".into(),
        }))
    }

    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<TopicMessage, Status>> + Send>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let request = request.into_inner();
        let receiver = self
            .topics
            .subscribe(&request.topic, &request.r#type)
            .map_err(Status::invalid_argument)?;
        let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
            Ok(message) => Some(Ok(TopicMessage {
                topic: message.topic.clone(),
                r#type: message.msg_type.clone(),
                json: message.json.clone(),
            })),
            Err(BroadcastStreamRecvError::Lagged(skipped)) => {
                eprintln!("gRPC subscriber lagged; skipped {skipped} messages");
                None
            }
        });
        Ok(Response::new(Box::pin(stream)))
    }
}
