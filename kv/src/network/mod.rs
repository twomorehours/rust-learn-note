use std::marker::PhantomData;

use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::{read_frame, FrameCodec, KvError};

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
