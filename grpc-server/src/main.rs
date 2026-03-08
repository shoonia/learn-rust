use std::error::Error;
use tonic::{Request, Response, Status, transport::Server};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub mod proto {
    tonic::include_proto!("com.servers");
}

use crate::proto::greeter_server::Greeter;
use proto::greeter_server::GreeterServer;
use proto::{HelloRequest, HelloResponse};

#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[tonic::async_trait]
impl Greeter for GreeterImpl {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let req = request.into_inner();

        let reponse = HelloResponse {
            reply: format!("Hello, {}!", req.name),
        };

        Ok(Response::new(reponse))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let server = GreeterImpl::default();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_headers(Any)
        .allow_methods(Any);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(GreeterServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
