use crate::net::{net_client::NetClient, GetNetRequest};

pub mod net {
    tonic::include_proto!("net");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NetClient::connect("http://localhost:50051").await?;

    let request = tonic::Request::new(GetNetRequest {
        url: "http://ehr.neusoft.com".into(),
    });

    let response = client.get(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
