use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

pub struct Shared<T> {
    q: Mutex<VecDeque<T>>,
    recv_waiter: Condvar,
    send_waiter: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
    cap: usize,
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::SeqCst);
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if 1 == self.shared.senders.fetch_sub(1, Ordering::SeqCst) {
            self.shared.recv_waiter.notify_all();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    local: VecDeque<T>,
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        self.shared.receivers.fetch_add(1, Ordering::SeqCst);
        Self {
            shared: self.shared.clone(),
            local: VecDeque::new(),
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::SeqCst);
    }
}

pub fn bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    let shared: Arc<Shared<T>> = Arc::new(Shared {
        q: Mutex::new(VecDeque::new()),
        recv_waiter: Condvar::new(),
        send_waiter: Condvar::new(),
        senders: AtomicUsize::new(1),
        receivers: AtomicUsize::new(1),
        cap,
    });

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            local: VecDeque::new(),
        },
    )
}

impl<T> Sender<T> {
    pub fn send(&self, data: T) -> Result<()> {
        if self.shared.receivers.load(Ordering::SeqCst) == 0 {
            return Err(anyhow!("no receivers available"));
        }
        let mut q = self.shared.q.lock().unwrap();
        while q.len() == self.shared.cap {
            q = self.shared.send_waiter.wait(q).unwrap();
        }
        q.push_back(data);
        self.shared.recv_waiter.notify_one();
        Ok(())
    }

    pub fn total_queued_items(&self) -> usize {
        let mut q = self.shared.q.lock().unwrap();
        q.len()
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        if let Some(data) = self.local.pop_front() {
            return Ok(data);
        }
        let mut q = self.shared.q.lock().unwrap();
        loop {
            match q.pop_front() {
                Some(data) => {
                    self.shared.send_waiter.notify_one();
                    self.local.push_back(data);
                }
                None => {
                    if let Some(data) = self.local.pop_front() {
                        return Ok(data);
                    }
                    if self.shared.senders.load(Ordering::SeqCst) == 0 {
                        return Err(anyhow!("no senders available"));
                    } else {
                        q = self.shared.recv_waiter.wait(q).unwrap();
                    }
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

#[test]
fn multiple_senders_should_work() {
    let (mut s, mut r) = bounded(100);
    let mut s1 = s.clone();
    let mut s2 = s.clone();
    let t = thread::spawn(move || {
        s.send(1).unwrap();
    });
    let t1 = thread::spawn(move || {
        s1.send(2).unwrap();
    });
    let t2 = thread::spawn(move || {
        s2.send(3).unwrap();
    });
    for handle in [t, t1, t2] {
        handle.join().unwrap();
    }

    let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
    // 在这个测试里，数据到达的顺序是不确定的，所以我们排个序再 assert
    result.sort();

    assert_eq!(result, [1, 2, 3]);
}

#[test]
fn receiver_should_be_blocked_when_nothing_to_read() {
    let (mut s, r) = bounded(100);
    let mut s1 = s.clone();
    thread::spawn(move || {
        for (idx, i) in r.into_iter().enumerate() {
            // 如果读到数据，确保它和发送的数据一致
            assert_eq!(idx, i);
        }
        eprintln!("recv dropped");
        // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
        assert!(false);
    });

    thread::spawn(move || {
        for i in 0..100usize {
            s.send(i).unwrap();
        }
    });

    // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
    thread::sleep(Duration::from_millis(1));

    // 再次发送数据，唤醒消费者
    for i in 100..200usize {
        s1.send(i).unwrap();
    }

    // 留点时间让 receiver 处理
    thread::sleep(Duration::from_millis(1));

    // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
    assert_eq!(s1.total_queued_items(), 0);
}

#[test]
fn last_sender_drop_should_error_when_receive() {
    let (s, mut r) = bounded(100);
    let s1 = s.clone();
    let senders = [s, s1];
    let total = senders.len();

    // sender 即用即抛
    for mut sender in senders {
        thread::spawn(move || {
            sender.send("hello").unwrap();
            // sender 在此被丢弃
        })
        .join()
        .unwrap();
    }

    // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
    for _ in 0..total {
        r.recv().unwrap();
    }

    // 然而，读取更多数据时会出错
    assert!(r.recv().is_err());
}

#[test]
fn receiver_drop_should_error_when_send() {
    let (mut s1, mut s2) = {
        let (s, _) = bounded(100);
        let s1 = s.clone();
        let s2 = s.clone();
        (s1, s2)
    };

    assert!(s1.send(1).is_err());
    assert!(s2.send(1).is_err());
}

#[test]
fn receiver_shall_be_notified_when_all_senders_exit() {
    let (s, mut r) = bounded::<usize>(100);
    // 用于两个线程同步
    let (mut sender, mut receiver) = bounded::<usize>(100);
    let t1 = thread::spawn(move || {
        // 保证 r.recv() 先于 t2 的 drop 执行
        sender.send(0).unwrap();
        assert!(r.recv().is_err());
    });

    thread::spawn(move || {
        receiver.recv().unwrap();
        drop(s);
    });

    t1.join().unwrap();
}

#[test]
fn channel_fast_path_should_work() {
    let (mut s, mut r) = bounded(100);
    for i in 0..10usize {
        s.send(i).unwrap();
    }

    assert!(r.local.is_empty());
    // 读取一个数据，此时应该会导致 swap，cache 中有数据
    assert_eq!(0, r.recv().unwrap());
    // 还有 9 个数据在 cache 中
    assert_eq!(r.local.len(), 9);
    // 在 queue 里没有数据了
    assert_eq!(s.total_queued_items(), 0);

    // 从 cache 里读取剩下的数据
    for (idx, i) in r.into_iter().take(9).enumerate() {
        assert_eq!(idx + 1, i);
    }
}
