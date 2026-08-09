use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic_web::GrpcWebLayer;

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

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
