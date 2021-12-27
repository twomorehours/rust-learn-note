// // 总结
// // 1. 当定义一个trait时， 不确定的类型的处理方式。
// //     当一个实现类实现trait时，这个类型的取值只有一个的情况下用关联类型。(一个类型只能实现一个trait一次 所以其内部关联类型也只能指定一次)
// //     当一个实现类实现trait时，这个类型的取值可能有多个用泛型。（一个类型实现了多个trait）
// 2. From/Into 从值到值 From在target实现 source自动实现Into
// 3. AsRef/AsMut 从引用到引用 在source实现

// // 问题
// // 1. 对于 Add trait，如果我们不用泛型，把 Rhs 作为 Add trait 的关联类型，可以么？为什么？
// //    不可以。 因为一个数据结构对一个trait的一个关联类型只能指定一次。
// // 2. 如下代码能编译通过么，为什么？
// //    不能 因为trait object会擦掉Self类型信息。导致Self未知大小。
// // use std::{fs::File, io::Write};
// // fn main() {
// //     let mut f = File::create("/tmp/test_write_trait").unwrap();
// //     let w: &mut dyn Write = &mut f;
// //     w.write_all(b"hello ").unwrap();
// //     let w1 = w.by_ref();
// //     w1.write_all(b"world").unwrap();
// // }

// // use std::io::Write;

// // #[derive(Default, Debug)]
// // struct MyBufWriter {
// //     buf: Vec<u8>,
// // }

// // impl Write for MyBufWriter {
// //     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
// //         self.buf.extend_from_slice(buf);
// //         Ok(buf.len())
// //     }

// //     fn flush(&mut self) -> std::io::Result<()> {
// //         Ok(())
// //     }
// // }

// // fn main() {
// //     let mut mw = MyBufWriter::default();
// //     mw.write_all("hello world".as_bytes()).unwrap();
// //     println!("{:?}", mw);
// // }

// 1.Vec 可以实现 Copy trait 么？为什么？
// 不能 因为Vec关联着堆上的内存 并实现了Drop 如果实现了Copy 可能会导致use after free | double free
// 2. 在使用 Arc<Mutex<T>> 时，为什么下面这段代码可以直接使用 shared.lock()？
// 因为Deref成了 &Mutex<T>
// use std::sync::{Arc, Mutex};
// let shared = Arc::new(Mutex::new(1));
// let mut g = shared.lock().unwrap();
// *g += 1;

// use std::ops::Add;

// #[derive(Debug, Copy, Clone)]
// struct Complex {
//     real: f64,
//     imagine: f64,
// }

// impl Complex {
//     pub fn new(real: f64, imagine: f64) -> Self {
//         Self { real, imagine }
//     }
// }

// // 对 Complex 类型的实现
// impl Add for Complex {
//     type Output = Self;

//     // 注意 add 第一个参数是 self，会移动所有权
//     fn add(self, rhs: Self) -> Self::Output {
//         let real = self.real + rhs.real;
//         let imagine = self.imagine + rhs.imagine;
//         Self::new(real, imagine)
//     }
// }

// fn main() {
//     let c1 = Complex::new(1.0, 1f64);
//     let c2 = Complex::new(2 as f64, 3.0);
//     println!("{:?}", c1 + c2);
//     // c1、c2 已经被移动，所以下面这句无法编译
//     println!("{:?}", c1 + c2);
// }

// struct SentenceIter<'a> {
//     s: &'a mut &'a str,
//     delimiter: char,
// }

// impl<'a> SentenceIter<'a> {
//     pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
//         Self { s, delimiter }
//     }
// }

// impl<'a> Iterator for SentenceIter<'a> {
//     type Item = &'a str;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.s.len() == 0 {
//             return None;
//         }
//         match self.s.find(self.delimiter) {
//             Some(idx) => {
//                 let next = &self.s[..idx + self.delimiter.len_utf8()];
//                 *self.s = &self.s[idx + self.delimiter.len_utf8()..];
//                 Some(next)
//             }
//             None => {
//                 let next = &self.s[..];
//                 *self.s = "";
//                 Some(next)
//             }
//         }
//     }
// }

// #[test]
// fn it_works() {
//     let mut s = "This is the 1st sentence. This is the 2nd sentence.";
//     let mut iter = SentenceIter::new(&mut s, '.');
//     assert_eq!(iter.next(), Some("This is the 1st sentence."));
//     assert_eq!(iter.next(), Some(" This is the 2nd sentence."));
//     assert_eq!(iter.next(), None);
// }

// fn main() {
//     let mut s = "a。 b。 c";
//     let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
//     println!("sentences: {:?}", sentences);
// }



fn main(){

}