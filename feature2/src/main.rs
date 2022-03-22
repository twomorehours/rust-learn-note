// feature是一种可选依赖的功能
// 每个feature对应一个mod 可以指定一个开启一个feature就默认开启指定的feature
// 使用方在使用时指定需要依赖的feature 不指定的feature不会被编译
// default = [] 表示默认开启的feature 可用default-featrues = false 关闭

fn main() {
    println!("Hello, world!");
    feature1::abc::calc(1, 2);
}
