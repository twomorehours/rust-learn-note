// // // 总结
// // // 1. 当定义一个trait时， 不确定的类型的处理方式。
// // //     当一个实现类实现trait时，这个类型的取值只有一个的情况下用关联类型。(一个类型只能实现一个trait一次 所以其内部关联类型也只能指定一次)
// // //     当一个实现类实现trait时，这个类型的取值可能有多个用泛型。（一个类型实现了多个trait）
// // 2. From/Into 从值到值 From在target实现 source自动实现Into
// // 3. AsRef/AsMut 从引用到引用 在source实现
// // 4. enum的tag占用1byte tag + padding + val 需要和8bytes对其 0 <= padding <= 7
// // 5. 给一个enum实现thiserror 然后在每个可能返回错误的地方都使用这个enum类型接收
// //    可以返回自定义的错误 也可以返回对其他错误的包装(#[from]error) 因为enum内部每个类型的值都可以作为enum类型的值
// // 6. 逐层考虑 每层的值如何组织 多对一       每层的值如何解析 一对多

// // // 问题
// // // 1. 对于 Add trait，如果我们不用泛型，把 Rhs 作为 Add trait 的关联类型，可以么？为什么？
// // //    不可以。 因为一个数据结构对一个trait的一个关联类型只能指定一次。
// // // 2. 如下代码能编译通过么，为什么？
// // //    不能 因为trait object会擦掉Self类型信息。导致Self未知大小。
// // // use std::{fs::File, io::Write};
// // // fn main() {
// // //     let mut f = File::create("/tmp/test_write_trait").unwrap();
// // //     let w: &mut dyn Write = &mut f;
// // //     w.write_all(b"hello ").unwrap();
// // //     let w1 = w.by_ref();
// // //     w1.write_all(b"world").unwrap();
// // // }
// // 3. 你知道 Cow<[u8]> 和 Cow 的大小么？试着打印一下看看。想想，为什么它的大小是这样呢？
// // Cow<[u8]> 可能是 &[u8]或Vec<u8> 取大的 Vec<u8> + tag + padding(7) = 32
// // Cow<str>  可能是 &str或String   取大的 String + tag + padding(7) = 32

// // // use std::io::Write;

// // // #[derive(Default, Debug)]
// // // struct MyBufWriter {
// // //     buf: Vec<u8>,
// // // }

// // // impl Write for MyBufWriter {
// // //     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
// // //         self.buf.extend_from_slice(buf);
// // //         Ok(buf.len())
// // //     }

// // //     fn flush(&mut self) -> std::io::Result<()> {
// // //         Ok(())
// // //     }
// // // }

// // // fn main() {
// // //     let mut mw = MyBufWriter::default();
// // //     mw.write_all("hello world".as_bytes()).unwrap();
// // //     println!("{:?}", mw);
// // // }

// // 1.Vec 可以实现 Copy trait 么？为什么？
// // 不能 因为Vec关联着堆上的内存 并实现了Drop 如果实现了Copy 可能会导致use after free | double free
// // 2. 在使用 Arc<Mutex<T>> 时，为什么下面这段代码可以直接使用 shared.lock()？
// // 因为Deref成了 &Mutex<T>
// // use std::sync::{Arc, Mutex};
// // let shared = Arc::new(Mutex::new(1));
// // let mut g = shared.lock().unwrap();
// // *g += 1;

// // use std::ops::Add;

// // #[derive(Debug, Copy, Clone)]
// // struct Complex {
// //     real: f64,
// //     imagine: f64,
// // }

// // impl Complex {
// //     pub fn new(real: f64, imagine: f64) -> Self {
// //         Self { real, imagine }
// //     }
// // }

// // // 对 Complex 类型的实现
// // impl Add for Complex {
// //     type Output = Self;

// //     // 注意 add 第一个参数是 self，会移动所有权
// //     fn add(self, rhs: Self) -> Self::Output {
// //         let real = self.real + rhs.real;
// //         let imagine = self.imagine + rhs.imagine;
// //         Self::new(real, imagine)
// //     }
// // }

// // fn main() {
// //     let c1 = Complex::new(1.0, 1f64);
// //     let c2 = Complex::new(2 as f64, 3.0);
// //     println!("{:?}", c1 + c2);
// //     // c1、c2 已经被移动，所以下面这句无法编译
// //     println!("{:?}", c1 + c2);
// // }

// // struct SentenceIter<'a> {
// //     s: &'a mut &'a str,
// //     delimiter: char,
// // }

// // impl<'a> SentenceIter<'a> {
// //     pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
// //         Self { s, delimiter }
// //     }
// // }

// // impl<'a> Iterator for SentenceIter<'a> {
// //     type Item = &'a str;

// //     fn next(&mut self) -> Option<Self::Item> {
// //         if self.s.len() == 0 {
// //             return None;
// //         }
// //         match self.s.find(self.delimiter) {
// //             Some(idx) => {
// //                 let next = &self.s[..idx + self.delimiter.len_utf8()];
// //                 *self.s = &self.s[idx + self.delimiter.len_utf8()..];
// //                 Some(next)
// //             }
// //             None => {
// //                 let next = &self.s[..];
// //                 *self.s = "";
// //                 Some(next)
// //             }
// //         }
// //     }
// // }

// // #[test]
// // fn it_works() {
// //     let mut s = "This is the 1st sentence. This is the 2nd sentence.";
// //     let mut iter = SentenceIter::new(&mut s, '.');
// //     assert_eq!(iter.next(), Some("This is the 1st sentence."));
// //     assert_eq!(iter.next(), Some(" This is the 2nd sentence."));
// //     assert_eq!(iter.next(), None);
// // }

// // fn main() {
// //     let mut s = "a。 b。 c";
// //     let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
// //     println!("sentences: {:?}", sentences);
// // }

// // fn main(){

// // }

// use std::{
//     collections::LinkedList,
//     fmt::{Debug, Display},
//     ops::{Deref, DerefMut, Index},
// };
// struct List<T>(LinkedList<T>);

// impl<T> Deref for List<T> {
//     type Target = LinkedList<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for List<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl<T> Default for List<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

// impl<T> Index<isize> for List<T> {
//     type Output = T;

//     fn index(&self, index: isize) -> &Self::Output {
//         let real_index = if index >= 0 {
//             index as usize % self.0.len()
//         } else {
//             let reverse = -index as usize % self.0.len();
//             if reverse == 0 {
//                 0
//             } else {
//                 self.0.len() - reverse
//             }
//         };
//         self.0.iter().skip(real_index).next().unwrap()
//     }
// }

// #[test]
// fn it_works() {
//     let mut list: List<u32> = List::default();
//     for i in 0..16 {
//         list.push_back(i);
//     }

//     assert_eq!(list[0], 0);
//     assert_eq!(list[5], 5);
//     assert_eq!(list[15], 15);
//     assert_eq!(list[16], 0);
//     assert_eq!(list[-1], 15);
//     assert_eq!(list[128], 0);
//     assert_eq!(list[-128], 0);
// }

// use std::str;

// struct MiniString {
//     len: u8,
//     data: [u8; 30],
// }

// impl MiniString {
//     fn new(s: impl AsRef<str>) -> Self {
//         let bytes = s.as_ref().as_bytes();
//         let mut data = [0u8; 30];
//         // 用slice的方法
//         data[..bytes.len()].copy_from_slice(bytes);
//         Self {
//             len: bytes.len() as u8,
//             data,
//         }
//     }

//     fn len(&self) -> usize {
//         self.len as usize
//     }

//     fn push_str(&mut self, string: &str) {
//         self.data[self.len as usize..self.len as usize + string.len()]
//             .copy_from_slice(string.as_bytes());
//         self.len += string.len() as u8;
//     }
// }

// impl Deref for MiniString {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         str::from_utf8(&self.data[..self.len as usize]).unwrap()
//     }
// }

// impl DerefMut for MiniString {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         let len = self.len;
//         str::from_utf8_mut(&mut self.data[..len as usize]).unwrap()
//     }
// }

// impl Debug for MiniString {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("MiniString")
//             .field("len", &self.len)
//             .field("data", &self.data)
//             .finish()
//     }
// }

// #[derive(Debug)]
// enum MyString {
//     Mini(MiniString),
//     Standard(String),
// }

// impl From<&str> for MyString {
//     fn from(val: &str) -> Self {
//         if val.len() <= 30 {
//             Self::Mini(MiniString::new(val))
//         } else {
//             Self::Standard(val.to_string())
//         }
//     }
// }

// impl From<String> for MyString {
//     fn from(val: String) -> Self {
//         MyString::Standard(val)
//     }
// }

// impl MyString {
//     fn len(&self) -> usize {
//         match self {
//             MyString::Mini(s) => s.len(),
//             MyString::Standard(s) => s.len(),
//         }
//     }

//     pub fn push_str(&mut self, string: &str) {
//         match self {
//             MyString::Mini(s) => {
//                 if s.len() + string.len() > 30 {
//                     let mut standard = s.deref().to_string();
//                     standard.push_str(string);
//                     *self = MyString::Standard(standard);
//                 } else {
//                     s.push_str(string);
//                 }
//             }
//             MyString::Standard(s) => {
//                 s.push_str(string);
//             }
//         }
//     }
// }

// impl Display for MyString {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.deref())
//     }
// }

// impl Deref for MyString {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         match self {
//             MyString::Mini(s) => s.deref(),
//             MyString::Standard(s) => s.deref(),
//         }
//     }
// }

// fn main() {
//     // let len1 = std::mem::size_of::<MyString>();
//     // let len2 = std::mem::size_of::<MiniString>();
//     // println!("Len: MyString {}, MiniString {}", len1, len2);

//     // let s1: MyString = "hello world".into();
//     // let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

//     // // debug 输出
//     // println!("s1: {:?}, s2: {:?}", s1, s2);
//     // // display 输出
//     // println!(
//     //     "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
//     //     s1,
//     //     s1.len(),
//     //     s1.chars().count(),
//     //     s2,
//     //     s2.len(),
//     //     s2.chars().count()
//     // );

//     // // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
//     // assert!(s1.ends_with("world"));
//     // assert!(s2.starts_with("这"));

//     let mut s1: MyString = "hello world".into();
//     s1.push_str("hello world");
//     println!("s1: {:?}", s1);
//     s1.push_str("hello world hello world hello world hello world hello world");
//     println!("s1: {:?}", s1);
// }

fn main() {}
