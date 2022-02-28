// 传统
// 每个flag都是一个常数
// 用一个可变的值保存flag |= curr    &= !curr     & curr == curr

// bitflags
// 每一个flag都是一个定义struct类型的值
// 用一个可变的struct类型的值保存flag flag.insert flag.remove flag.contains

use bitflags::bitflags;

bitflags! {
    struct Permissions: u32 {
        const READ = 0b00000001;
        const WRITE = 0b00000010;
        const EXECUTE = 0b00000100;
    }
}

fn main() {
    let mut perm = Permissions::empty();
    println!("{:?}", perm);
    assert!(!perm.contains(Permissions::READ));
    println!("{}", perm.bits());
    perm.insert(Permissions::READ);
    assert!(perm.contains(Permissions::READ));
    println!("{}", perm.bits());
    perm.remove(Permissions::READ);
    assert!(!perm.contains(Permissions::READ));
    println!("{}", perm.bits());
}
