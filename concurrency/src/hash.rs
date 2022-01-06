use anyhow::Result;
use futures::{Sink, SinkExt, StreamExt};
use std::{collections::hash_map::DefaultHasher, hash::Hasher, thread};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_util::codec::{Framed, LinesCodec};

pub async fn start() -> Result<()> {
    let (sender, mut receiver) = mpsc::unbounded_channel::<(oneshot::Sender<u64>, String)>();

    thread::spawn(move || {
        while let Some((sender, data)) = receiver.blocking_recv() {
            let mut hasher = DefaultHasher::new();
            hasher.write(data.as_bytes());
            if let Err(e) = sender.send(hasher.finish()) {
                eprintln!("send error: {}", e);
            }
        }
    });

    let listener = TcpListener::bind("localhost:9527").await?;
    loop {
        let sender = sender.clone();
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut stream = Framed::new(stream, LinesCodec::new());
            while let Some(Ok(data)) = stream.next().await {
                let (sender1, receiver1) = oneshot::channel();
                sender.send((sender1, data))?;
                let result = receiver1.await?;
                stream.send(result.to_string()).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}
