use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#changed logs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:6969").await?;

    let request = tonic::Request::new(
        HelloRequest {
            name: "Aziz".into(),
        }
    );

    let response = client.say_hello(request).await?;

    println!("Response={:?}", response.into_inner().message);
    Ok(())
}