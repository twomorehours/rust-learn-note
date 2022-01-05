use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kv::{
    AsyncProstStream, CommandRequest, CommandResponse, Memtable, ProstServerStream, Service,
    ServiceInner,
};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{error, info};

// #[tokio::main]
// async fn main() -> Result<()> {
//     tracing_subscriber::fmt::init();

//     let service: Service<_> = ServiceInner::new(Memtable::new()).into();

//     let listener = TcpListener::bind("localhost:9527").await?;

//     loop {
//         let (stream, addr) = listener.accept().await?;
//         info!("accept stream from {:?}", addr);
//         let service = service.clone();
//         tokio::spawn(async move {
//             let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

//             while let Some(Ok(data)) = stream.next().await {
//                 let req = data.try_into()?;
//                 info!("receive request {:?}", req);
//                 let resp = service.execute(req);
//                 if let Err(e) = stream.send(resp.into()).await {
//                     error!("send resp error {:?}", e);
//                 }
//             }

//             Ok::<_, anyhow::Error>(())
//         });
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     tracing_subscriber::fmt::init();

//     let service: Service<_> = ServiceInner::new(Memtable::new()).into();

//     let listener = TcpListener::bind("localhost:9527").await?;

//     loop {
//         let (stream, addr) = listener.accept().await?;
//         info!("accept stream from {:?}", addr);
//         let service = service.clone();
//         tokio::spawn(async move {
//             let mut stream: AsyncProstStream<_, CommandRequest, CommandResponse> =
//                 AsyncProstStream::new(stream);

//             while let Ok(req) = stream.next().await {
//                 info!("receive request {:?}", req);
//                 let resp = service.execute(req);
//                 if let Err(e) = stream.send(resp).await {
//                     error!("send resp error {:?}", e);
//                 }
//             }

//             Ok::<_, anyhow::Error>(())
//         });
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("localhost:9527").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("accept stream from {:?}", addr);
        tokio::spawn(async move {
            let mut stream = ProstServerStream::new(stream, Memtable::new());
            if let Err(e) = stream.process().await {
                error!("process error {:?}", e);
            }

            Ok::<_, anyhow::Error>(())
        });
    }
}
