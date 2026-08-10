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

#[derive(Default)]
struct RobotServer;

#[tonic::async_trait]
impl RobotService for RobotServer {
    async fn set_joy(&self, request: Request<JoyRequest>) -> Result<Response<Empty>, Status> {
        let joy = request.into_inner();

        println!("left_x={}, left_y={}", joy.left_x, joy.left_y);

        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    let service = RobotServiceServer::new(RobotServer);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

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
        .await?;

    Ok(())
}
