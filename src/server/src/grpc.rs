use crate::robot::{Empty, JoyRequest, JoyResponse, robot_service_server::RobotService};
use crate::ros::{JoyData, JoyNode};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};
pub struct RobotServer {
    pub ros: Arc<JoyNode>,
    pub joy_tx: broadcast::Sender<JoyData>,
}

#[tonic::async_trait]
impl RobotService for RobotServer {
    async fn set_joy(&self, request: Request<JoyRequest>) -> Result<Response<Empty>, Status> {
        let joy_req = request.into_inner();

        println!("left_x={}, left_y={}", joy_req.left_x, joy_req.left_y);
        let joy = JoyData {
            axes: vec![
                joy_req.left_x,
                joy_req.left_y,
                joy_req.right_x,
                joy_req.right_y,
            ],
            buttons: vec![],
        };
        self.ros
            .publish(joy)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }
    type SubscribeJoyStream = Pin<Box<dyn Stream<Item = Result<JoyResponse, Status>> + Send>>;

    async fn subscribe_joy(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<Self::SubscribeJoyStream>, Status> {
        println!("Subscribe called");
        let receiver = self.joy_tx.subscribe();
        let stream = BroadcastStream::new(receiver).map(|res| match res {
            Ok(data) => Ok(JoyResponse {
                left_x: data.axes[0],
                left_y: data.axes[1],
                right_x: data.axes[2],
                right_y: data.axes[3],
            }),
            Err(err) => Err(Status::internal(err.to_string())),
        });
        Ok(Response::new(Box::pin(stream)))
    }
}
