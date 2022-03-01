// 传统
// 每个flag都是一个常数
// 用一个可变的值保存flag |= curr    &= !curr     & curr == curr

// bitflags
// 每一个flag都是一个定义struct类型的值
// 用一个可变的struct类型的值保存flag flag.insert flag.remove flag.contains

use base64::decode;
use bitflags::bitflags;

bitflags! {
    struct Permissions: u32 {
        const READ = 0b00000001;
        const WRITE = 0b00000010;
        const EXECUTE = 0b00000100;
    }
}

fn main() {
    // let mut perm = Permissions::empty();
    // println!("{:?}", perm);
    // assert!(!perm.contains(Permissions::READ));
    // println!("{}", perm.bits());
    // perm.insert(Permissions::READ | Permissions::WRITE);
    // assert!(perm.contains(Permissions::READ));
    // println!("{}", perm.bits());
    // perm.remove(Permissions::READ);
    // assert!(!perm.contains(Permissions::READ));
    // println!("{}", perm.bits());

    // 考虑一个slice是应该考虑 头(ptr)、尾(cap)、len(curr pos)
    // split就是产生一个新的view 从ptr是0 cap到len,len为最后  ptr是len cap是cap-len len为0
    // 是用于写一段切下去的场景

    // split_off和split_to是相反的 split_off是保留前面一段 split_to是保留后面一段
    // 前面一段len=min(len,max) 后面一段len=if(len>at){len-at}else{0}

    use bytes::{BufMut, BytesMut};

    let mut buf = BytesMut::with_capacity(1024);

    buf.put(&b"hello"[..]);

    let a = buf.split_off(10);

    eprintln!(
        "{} {} {} {}",
        a.len(),
        a.capacity(),
        buf.len(),
        buf.capacity()
    );

    // buf.put_u16(1234);

    // let a = buf.split();
    // eprintln!(
    //     "{} {} {} {}",
    //     a.len(),
    //     a.capacity(),
    //     buf.len(),
    //     buf.capacity()
    // );
    // assert_eq!(a, b"hello world\x04\xD2"[..]);

    // buf.put(&b"goodbye world"[..]);

    // let b = buf.split();
    // assert_eq!(b, b"goodbye world"[..]);

    // assert_eq!(buf.capacity(), 998);

    // println!("{}", i32::MAX + 1);
}
