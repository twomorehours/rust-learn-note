mod command;
use std::sync::Arc;

pub use command::*;

use crate::{command_request::RequestData, CommandRequest, CommandResponse, Memtable, Storage};

pub struct Service<Store = Memtable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, request: CommandRequest) -> CommandResponse {
        dispatch(&self.inner.store, request)
    }
}

impl<T> Clone for Service<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

fn dispatch(store: &impl Storage, request: CommandRequest) -> CommandResponse {
    match request.request_data {
        Some(RequestData::Hget(hget)) => hget.execute(store),
        Some(RequestData::Hset(hset)) => hset.execute(store),
        Some(RequestData::Hgetall(hgetall)) => hgetall.execute(store),
        _ => unreachable!(),
    }
}

struct ServiceInner<Store> {
    store: Store,
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::{Memtable, Value};

    #[test]
    fn service_should_works() {
        // 我们需要一个 service 结构至少包含 Storage
        let service = Service::new(Memtable::default());

        // service 可以运行在多线程环境下，它的 clone 应该是轻量级的
        let cloned = service.clone();

        // 创建一个线程，在 table t1 中写入 k1, v1
        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        // 在当前线程下读取 table t1 的 k1，应该返回 v1
        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

// 测试成功返回的结果
#[cfg(test)]
pub fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// 测试失败返回的结果
#[cfg(test)]
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}
