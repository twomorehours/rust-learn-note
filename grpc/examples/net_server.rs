use anyhow::Result;
use net::{
    net_server::{Net, NetServer},
    GetNetRequest, GetNetResponse,
};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

mod net {
    tonic::include_proto!("net");
}

#[derive(Debug, Default)]
struct NetGetter {
    cache: RwLock<HashMap<String, String>>,
}

#[tonic::async_trait]
impl Net for NetGetter {
    async fn get(
        &self,
        request: Request<GetNetRequest>,
    ) -> Result<Response<GetNetResponse>, Status> {
        let request = request.into_inner();
        let rg = self.cache.read().await;
        let text = match rg.get(&request.url) {
            Some(text) => text.to_owned(),
            None => {
                drop(rg);
                // get url text
                let text = reqwest::get(&request.url)
                    .await
                    .map_err(|_| Status::internal("http error"))?
                    .text()
                    .await
                    .map_err(|_| Status::internal("http error"))?;

                let mut wg = self.cache.write().await;
                wg.insert(request.url, text.clone());
                text
            }
        };
        Ok(Response::new(GetNetResponse { text }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    let net_getter = NetGetter::default();

    Server::builder()
        .add_service(NetServer::new(net_getter))
        .serve(addr)
        .await?;

    Ok(())
}
