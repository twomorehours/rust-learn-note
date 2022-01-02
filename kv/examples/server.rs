use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, CommandResponse, Memtable, Service, ServiceInner};
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service: Service<_> = ServiceInner::new(Memtable::new()).into();

    let listener = TcpListener::bind("localhost:9527").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("accept stream from {:?}", addr);
        let service = service.clone();
        tokio::spawn(async move {
            let mut stream: AsyncProstStream<_, CommandRequest, CommandResponse, _> =
                AsyncProstStream::from(stream).for_async();
            while let Some(Ok(req)) = stream.next().await {
                info!("receive request {:?}", req);
                let resp = service.execute(req);
                if let Err(e) = stream.send(resp).await {
                    error!("send resp error {:?}", e);
                }
            }
        });
    }
}
