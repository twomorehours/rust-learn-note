use crate::{CommandRequest, CommandResponse, KvError};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
use prost::Message;
use std::io::prelude::*;
use tokio::io::{AsyncRead, AsyncReadExt};

// 长度整个占用 4 个字节
const LEN_LEN: usize = 4;
/// 长度占 31 bit，所以最大的 frame 是 2G
const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;
/// 如果 payload 超过了 1436 字节，就做压缩
const COMPRESSION_LIMIT: usize = 1436;
/// 代表压缩的 bit（整个长度 4 字节的最高位）
const COMPRESSION_BIT: usize = 1 << 31;

pub trait FrameCodec
where
    Self: Message + Sized + Default,
{
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let len = self.encoded_len();

        if len < COMPRESSION_LIMIT {
            // no compressed
            buf.put_u32(len as _);
            self.encode(buf)?;
        } else {
            // with compressed
            let mut e = GzEncoder::new(Vec::with_capacity(len), Compression::default());
            e.write_all(&self.encode_to_vec())?;
            let compressed_bytes = e.finish()?;
            buf.put_u32((compressed_bytes.len() | COMPRESSION_BIT) as _);
            buf.put(Bytes::from(compressed_bytes));
        }

        Ok(())
    }
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError> {
        let header = buf.get_u32();
        let (len, compressed) = decode_header(header as _);
        if compressed {
            let mut writer = Vec::with_capacity(len * 2);
            let mut decoder = GzDecoder::new(writer);
            decoder.write_all(&buf[..len])?;
            writer = decoder.finish()?;
            Ok(Self::decode(&writer[..writer.len()])?)
        } else {
            Ok(Self::decode(&buf[..len])?)
        }
    }
}

impl FrameCodec for CommandResponse {}
impl FrameCodec for CommandRequest {}

fn decode_header(header: usize) -> (usize, bool) {
    let compressed = (header & COMPRESSION_BIT) == COMPRESSION_BIT;
    let len = header & !COMPRESSION_BIT;
    (len, compressed)
}

pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), KvError>
where
    S: AsyncRead + Send + Unpin,
{
    let header = stream.read_u32().await?;
    let (len, _) = decode_header(header as usize);
    buf.reserve(LEN_LEN + len);
    buf.put_u32(header);
    unsafe { buf.advance_mut(len) }
    stream.read_exact(&mut buf[LEN_LEN..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;
    use bytes::Bytes;

    #[test]
    fn command_request_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let cmd = CommandRequest::new_hdel("t1", "k1");
        cmd.encode_frame(&mut buf).unwrap();

        // 最高位没设置
        assert_eq!(is_compressed(&buf), false);

        let cmd1 = CommandRequest::decode_frame(&mut buf).unwrap();
        assert_eq!(cmd, cmd1);
    }

    #[test]
    fn command_response_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let values: Vec<Value> = vec![1.into(), "hello".into(), b"data".into()];
        let res: CommandResponse = values.into();
        res.encode_frame(&mut buf).unwrap();

        // 最高位没设置
        assert_eq!(is_compressed(&buf), false);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(res, res1);
    }

    #[test]
    fn command_response_compressed_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Value = Bytes::from(vec![0u8; COMPRESSION_LIMIT + 1]).into();
        let res: CommandResponse = value.into();
        res.encode_frame(&mut buf).unwrap();

        // 最高位设置了
        assert_eq!(is_compressed(&buf), true);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(res, res1);
    }

    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }

    struct DummyStream {
        buf: BytesMut,
    }

    impl AsyncRead for DummyStream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            // 看看 ReadBuf 需要多大的数据
            let len = buf.capacity();

            // split 出这么大的数据
            let data = self.get_mut().buf.split_to(len);

            // 拷贝给 ReadBuf
            buf.put_slice(&data);

            // 直接完工
            std::task::Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn read_frame_should_work() {
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_hdel("t1", "k1");
        cmd.encode_frame(&mut buf).unwrap();
        let mut stream = DummyStream { buf };

        let mut data = BytesMut::new();
        read_frame(&mut stream, &mut data).await.unwrap();

        let cmd1 = CommandRequest::decode_frame(&mut data).unwrap();
        assert_eq!(cmd, cmd1);
    }
}
