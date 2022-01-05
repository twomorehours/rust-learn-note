use std::marker::PhantomData;

use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tracing::{error, info};

use crate::{
    read_frame, CommandRequest, CommandResponse, FrameCodec, KvError, Service, ServiceInner,
    Storage,
};

pub struct AsyncProstStream<S, In, Out> {
    inner: S,
    _in: PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<S, In, Out> AsyncProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Send + Unpin,
    In: FrameCodec,
    Out: FrameCodec,
{
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            _in: PhantomData::default(),
            _out: PhantomData::default(),
        }
    }

    pub async fn next(&mut self) -> Result<In, KvError> {
        let mut buf = BytesMut::new();
        read_frame(&mut self.inner, &mut buf).await?;
        Ok(In::decode_frame(&mut buf)?)
    }

    pub async fn send(&mut self, out: Out) -> Result<(), KvError> {
        let mut buf = BytesMut::new();
        out.encode_frame(&mut buf)?;
        Ok(self.inner.write_all(&mut buf).await?)
    }
}

pub struct ProstServerStream<S, Store> {
    inner: AsyncProstStream<S, CommandRequest, CommandResponse>,
    service: Service<Store>,
}

impl<S, Store> ProstServerStream<S, Store>
where
    S: AsyncRead + AsyncWrite + Send + Unpin,
    Store: Storage,
{
    pub fn new(stream: S, store: Store) -> Self {
        Self {
            inner: AsyncProstStream::new(stream),
            service: ServiceInner::new(store).into(),
        }
    }

    pub async fn process(&mut self) -> Result<(), KvError> {
        while let Ok(req) = self.inner.next().await {
            info!("receive request {:?}", req);
            let resp = self.service.execute(req);
            if let Err(e) = self.inner.send(resp).await {
                error!("send resp error {:?}", e);
            }
        }

        Ok(())
    }
}

pub struct ProstClientStream<S> {
    inner: AsyncProstStream<S, CommandResponse, CommandRequest>,
}

impl<S> ProstClientStream<S>
where
    S: AsyncRead + AsyncWrite + Send + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            inner: AsyncProstStream::new(stream),
        }
    }

    pub async fn execute(&mut self, command: CommandRequest) -> Result<CommandResponse, KvError> {
        self.inner.send(command).await?;
        self.inner.next().await
    }
}
