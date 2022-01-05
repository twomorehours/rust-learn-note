mod abi;
pub use abi::*;

use crate::KvError;
use bytes::{Bytes, BytesMut};
use http::StatusCode;
use prost::Message;

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
}

impl TryFrom<BytesMut> for CommandRequest {
    type Error = KvError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Ok(Self::decode(value)?)
    }
}

impl From<CommandRequest> for Bytes {
    fn from(value: CommandRequest) -> Self {
        let mut buf = BytesMut::new();
        value.encode(&mut buf).unwrap();
        buf.into()
    }
}

impl TryFrom<BytesMut> for CommandResponse {
    type Error = KvError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Ok(Self::decode(value)?)
    }
}

impl From<CommandResponse> for Bytes {
    fn from(value: CommandResponse) -> Self {
        let mut buf = BytesMut::new();
        value.encode(&mut buf).unwrap();
        buf.into()
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            value: Some(value::Value::String(value.into())),
        }
    }
}

impl From<Bytes> for Value {
    fn from(values: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(values)),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(values: &[u8; N]) -> Self {
        Bytes::copy_from_slice(values).into()
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(p: (String, Value)) -> Self {
        Kvpair::new(p.0, p.1)
    }
}

impl From<KvError> for CommandResponse {
    fn from(err: KvError) -> Self {
        let mut resp = Self {
            status: StatusCode::OK.as_u16() as u32,
            message: err.to_string(),
            ..CommandResponse::default()
        };

        match err {
            KvError::NotFound(_, _) => {
                resp.status = StatusCode::NOT_FOUND.as_u16() as u32;
            }
            KvError::InvalidCommand(_) => {
                resp.status = StatusCode::BAD_REQUEST.as_u16() as u32;
            }
            _ => {}
        }

        resp
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        CommandResponse {
            values: vec![value],
            status: StatusCode::OK.as_u16() as u32,
            ..CommandResponse::default()
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(values: Vec<Value>) -> Self {
        CommandResponse {
            values,
            status: StatusCode::OK.as_u16() as u32,
            ..CommandResponse::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(pairs: Vec<Kvpair>) -> Self {
        CommandResponse {
            pairs,
            status: StatusCode::OK.as_u16() as u32,
            ..CommandResponse::default()
        }
    }
}

impl CommandResponse {
    pub fn ok() -> Self {
        CommandResponse {
            status: StatusCode::OK.as_u16() as u32,
            ..CommandResponse::default()
        }
    }
}
