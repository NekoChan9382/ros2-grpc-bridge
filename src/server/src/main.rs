use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_reflection::server::Builder;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod robot {
    tonic::include_proto!("ros_bridge");
}
mod grpc;
mod ros;

use grpc::grpc_manager::BridgeServer;
use ros::topic_manager::TopicManager;

use robot::ros_bridge_server::RosBridgeServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("ros_grpc_bridge")?;
    let topics = std::sync::Arc::new(TopicManager::new(node));

    let service = RosBridgeServer::new(BridgeServer { topics });

    let cert = tokio::fs::read("../../cert/server.cert").await?;
    let key = tokio::fs::read("../../cert/server.key").await?;
    let identity = Identity::from_pem(cert, key);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    tokio::spawn(async move {
        Server::builder()
            .tls_config(ServerTlsConfig::new().identity(identity))?
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
