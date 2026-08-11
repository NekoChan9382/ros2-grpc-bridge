use rclrs::{
    Context, CreateBasicExecutor, Executor, Publisher, RclrsError, RclrsErrorFilter, SpinOptions,
};
use sensor_msgs::msg::Joy;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod robot {
    tonic::include_proto!("robot");
}

use robot::{
    Empty, JoyRequest,
    robot_service_server::{RobotService, RobotServiceServer},
};

struct JoyNode {
    publisher: Arc<Publisher<Joy>>,
}

impl JoyNode {
    fn new(executor: &Executor) -> Result<Self, RclrsError> {
        let node = executor.create_node("grpc_bridge")?;
        let publisher = node.create_publisher::<Joy>("/joy")?;
        Ok(Self {
            publisher: Arc::new(publisher),
        })
    }
}

struct RobotServer {
    node: Arc<JoyNode>,
}

#[tonic::async_trait]
impl RobotService for RobotServer {
    async fn set_joy(&self, request: Request<JoyRequest>) -> Result<Response<Empty>, Status> {
        let joy_req = request.into_inner();

        println!("left_x={}, left_y={}", joy_req.left_x, joy_req.left_y);
        let mut joy = Joy::default();
        joy.axes = vec![
            joy_req.left_x,
            joy_req.left_y,
            joy_req.right_x,
            joy_req.right_y,
        ];
        joy.buttons = vec![];
        self.node
            .publisher
            .publish(joy)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = JoyNode::new(&executor)?;

    let service = RobotServiceServer::new(RobotServer {
        node: Arc::new(node),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    tokio::spawn(async move {
        Server::builder()
            .accept_http1(true)
            .layer(cors)
            .layer(GrpcWebLayer::new())
            .add_service(service)
            .add_service(
                Builder::configure()
                    .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                        "robot_descriptor"
                    ))
                    .build_v1()
                    .unwrap(),
            )
            .serve(addr)
            .await
    });
    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}
