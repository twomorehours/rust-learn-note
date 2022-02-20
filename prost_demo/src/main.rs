// frame是 分隔符+payload
// 常见的三种frame
// 1、 fixed 没有分割符 每次读固定大小
// 2. char分割 读到固定字符为止 适用于字符数据
// 3. length分割符 将payload的长度写入一个u32 写到payload之前作为分隔符 每次先读出de到payload大小 再读出payload 

// serde
// serde_derive 实现了Serialize和Deserialize 
// Serialize接受一个Serializer 将自身的value写入 
// Deserialize接受一个Deserializer 从中读出数据 转成自身的value
// Serializer Deserializer 由serde_xx 实现 如 serde_json serde_yaml
// json示例
// serde_json::to_string(value) => value.serialize(serializer) => serializer.to_string()
// serde_json::from(str) => Deserializer::from(str) => Type::deserialize(Deserializer) => value


use bytes::{BytesMut, Buf, BufMut};
use prost::Message;

mod pb;
use pb::*;

fn main() {
    // let shirt = Shirt::new("red", shirt::Size::Medium);
    // let mut buf = BytesMut::with_capacity(shirt.encoded_len());
    // let mut buf_with_de = BytesMut::with_capacity(shirt.encoded_len() + 4);
    // shirt.encode(&mut buf).unwrap();
    // buf_with_de.put_u32(shirt.encoded_len() as u32);
    // shirt.encode(&mut buf_with_de).unwrap();

    // eprintln!("encode_len: {:?}, de_len: {:?}", buf.len(), buf_with_de.get_u32());

    // let json = serde_json::to_string_pretty(&shirt).unwrap();
    // eprintln!("{:#?}", json);

    // let shirt1: Shirt = serde_json::from_str(&json).unwrap();
    // eprintln!("{:#?}", shirt1);


    let shirt = Shirt::new("red", shirt::Size::Medium);
    println!("pb: {}, json: {}", shirt.encoded_len(), serde_json::to_string_pretty(&shirt).unwrap().as_bytes().len());


}
