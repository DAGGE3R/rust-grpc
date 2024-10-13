use tonic::{Request, Response, Status};
use tonic::transport::Server;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug,Default)]
pub struct MyGreeter {

}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let reply = hello_world::HelloResponse {
            message : format!("Hello {} from Server", request.into_inner().name)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adress = "127.0.0.1:6969".parse()?;
    let greeter_service = MyGreeter::default();
    println!("GreeterServer listening on {}", adress);
    Server::builder().add_service(GreeterServer::new(greeter_service)).serve(adress).await?;

    Ok(())
}
