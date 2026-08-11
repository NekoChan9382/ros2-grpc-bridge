use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};
use std::sync::Arc;
use tokio::sync::broadcast;
use tonic::transport::Server;
use tonic_reflection::server::Builder;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod robot {
    tonic::include_proto!("robot");
}
mod grpc;
mod ros;

use grpc::RobotServer;
use ros::JoyNode;

use robot::robot_service_server::RobotServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let (joy_tx, _) = broadcast::channel(32);
    let node = JoyNode::new(&executor, joy_tx.clone())?;

    let service = RobotServiceServer::new(RobotServer {
        ros: Arc::new(node),
        joy_tx: joy_tx,
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
