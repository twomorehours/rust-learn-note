use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

// 总结
// 1. 想要用不可变引用取到可变引用就用内部可变指针。主要是值在被共享的时候。
// 2. Mutex的区别
//                    其他                         Rust
// 作用（场景）       互斥处理共享数据逻辑                   返回共享值的可变引用并保护值
// 实现    调用Mutex 满足通过 不满足阻塞           调用Mutex 满足返回引用 不满足阻塞

// 3. 如何实现Future
//   - 写async fn/block 会编译成 fn -> impl Future<Output = T>
//   - impl AsyncXXX trait 这种trait都有默认的实现Future 但是需要回调实现的poll
// 4. 如何调用Future
//   - 调用async函数获得Future .await
//   - 调用AsyncXXXExt trait 提供的返回Future的函数 .await
//   - 只有spawn才会创建task .await不会创建task 只是在当前task内调用future
//   - #[tokio::main]最开始就在一个task中执行

// 5. 自己写应该注意什么
//     - 会调用.await的函数用async
//     - .await会在当前task call一个future 这个future是一个状态机 当执行到最终状态的时候返回

// 6. aysnc await是什么
//    - async用于定义返回Future的函数 Fn() -> Future
//    - await用用于调用Future match f.poll() {Ready|Pending}
//    - 编译之后async和await都不存在了

// 7. 一次流程
//    - async或impl AsyncXXX得到一个能返回Future的函数
//    - 根据是否同步执行选择使用.await或者spawn
//    - 虽然真正调用的不是async函数本身而是其返回的futrue 但是future的还是写在async里面的 所以查问题是去async里面查
//    - 整个项目是由大量的这种流程组成的 到最后免疫完成后变成普通调用了

// 8. async fn创建一个能返回Future的函数 获取到Future根据同步异步决定如何调用

// 9. 同步函数是函数调函数 async是调用函数得到future之后再调用future

// use std::{
//     cell::{Cell, RefCell},
//     sync::{
//         atomic::{AtomicBool, AtomicUsize, Ordering},
//         Arc,
//     },
//     thread::{self, sleep},
//     time::{self, Duration},
// };

// pub struct Lock<T> {
//     locked: AtomicBool,
//     data: RefCell<T>,
// }

// impl<T> Lock<T> {
//     pub fn new(data: T) -> Self {
//         Self {
//             locked: AtomicBool::new(false),
//             data: RefCell::new(data),
//         }
//     }

//     pub fn lock(&self, f: impl FnOnce(&mut T)) {
//         loop {
//             if self.locked.load(Ordering::SeqCst) {
//                 continue;
//             }
//             if let Ok(false) =
//                 self.locked
//                     .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
//             {
//                 break;
//             }
//         }
//         f(&mut self.data.borrow_mut());
//         self.locked.store(false, Ordering::SeqCst);
//     }
// }

// unsafe impl<T> Sync for Lock<T> {}

// struct Semaphore {
//     count: AtomicUsize,
// }

// impl Semaphore {
//     pub fn new(count: usize) -> Self {
//         Self {
//             count: AtomicUsize::new(count),
//         }
//     }

//     pub fn acquire(&self) {
//         loop {
//             let curr = self.count.load(Ordering::SeqCst);
//             if curr == 0 {
//                 continue;
//             }
//             if let Ok(c) =
//                 self.count
//                     .compare_exchange(curr, curr - 1, Ordering::SeqCst, Ordering::SeqCst)
//             {
//                 if c == curr {
//                     break;
//                 }
//             }
//         }
//     }

//     pub fn release(&self) {
//         self.count.fetch_add(1, Ordering::SeqCst);
//     }
// }

// fn main() {
//     // println!("Hello, world!");
//     // let mut handles = Vec::new();
//     // let lock = Arc::new(Lock::new(0));
//     // for _ in 0..100 {
//     //     let lock = lock.clone();
//     //     handles.push(thread::spawn(move || {
//     //         lock.lock(|i| *i += 100);
//     //     }));
//     // }

//     // handles.into_iter().for_each(|h| {
//     //     h.join();
//     // });

//     // lock.lock(|i| println!("{}", i));

//     let mut handles = Vec::new();
//     let semaphore = Arc::new(Semaphore::new(2));
//     for _ in 0..10 {
//         let semaphore = semaphore.clone();
//         handles.push(thread::spawn(move || {
//             semaphore.acquire();
//             println!("Hello, world!");
//             sleep(Duration::from_secs(1));
//             semaphore.release();
//         }));
//     }

//     handles.into_iter().for_each(|h| {
//         h.join();
//     });

//     // lock.lock(|i| println!("{}", i));
// }

// fn main() {
//     let pair = Arc::new((Mutex::new(false), Condvar::new()));
//     let pair2 = pair.clone();
//     thread::spawn(move || {
//         thread::sleep(Duration::from_secs(1));
//         println!("sub execute");
//         let mut started = pair.0.lock().unwrap();
//         *started = true;
//         pair.1.notify_one();
//     });

//     let mut started = pair2.0.lock().unwrap();
//     while !*started {
//         // started = pair2.1.wait(started).unwrap();
//     }
//     println!("main execute")
// }

mod bound;
mod unbound;

mod hash;

mod chain;

#[tokio::main]
async fn main() {
    hash::start().await.unwrap();
}
