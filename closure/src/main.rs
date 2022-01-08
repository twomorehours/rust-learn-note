// 总结
// 1. 闭包代码会被编译成一个新类型和一个构建类型值得逻辑。看到一个闭包的时候就应该考虑这个类型的值如何在内存上分布以及实现的triat类型。
// 2. 调用闭包函数实际上就是调用struct的call()
// 3. 闭包成员会move出去的会实现FnOnce 闭包成员会被修改的会实现FnMut(+FnOnce) 不会move也不会修改的实现Fn(+FnMut)
// 4. 可以给闭包实现trait impl X for T  where T: Fn(A) -> B  实现的时候直接考虑T为trait(函数式调用) 不要考虑具体实现
// 5. Fn的只有一部分（未捕捉任何值）满足fn的限制。 当声明fn时只能传入fn和Fn中的那部分。 声明Fn/FnMut/FnOnce时，都可以传入fn。
// 6. 当声明函数（闭包）的时候，可以从最小的方位开始尝试（fn -> FnOnce）

// 思考
// 1. 下面的代码，闭包 c 相当于一个什么样的结构体？它的长度多大？代码的最后，main() 函数还能访问变量 name 么？为什么？

// struct Closure<'a, 'b> {
//     data: (i32, i32, i32, i32),
//     v: &'a [&'b str],
//     name: String,
// }
// 不能访问 因为move到闭包里面了 先move在clone()

// fn main() {
//     let name = String::from("Tyr");
//     let vec = vec!["Rust", "Elixir", "Javascript"];
//     let v = &vec[..];
//     let data = (1, 2, 3, 4);
//     let c = move || {
//         println!("data: {:?}", data);
//         println!("v: {:?}, name: {:?}", v, name.clone());
//     };
//     c();
//     // println!("{}", name);
//     // 请问在这里，还能访问 name 么？为什么？
// }

// 2. 为下面的代码添加实现，使其能够正常工作
// pub trait Executor {
//     fn execute(&self, cmd: &str) -> Result<String, &'static str>;
// }

// struct BashExecutor {
//     env: String,
// }

// impl Executor for BashExecutor {
//     fn execute(&self, cmd: &str) -> Result<String, &'static str> {
//         Ok(format!(
//             "fake bash execute: env: {}, cmd: {}",
//             self.env, cmd
//         ))
//     }
// }

// impl <T> Executor for T
// where
//     T: Fn(&str) -> Result<String, &'static str>,
// {
//     fn execute(&self, cmd: &str) -> Result<String, &'static str> {
//         self(cmd)
//     }
// }

// fn main() {
//     let env = "PATH=/usr/bin".to_string();

//     let cmd = "cat /etc/passwd";
//     let r1 = execute(cmd, BashExecutor { env: env.clone() });
//     println!("{:?}", r1);

//     let r2 = execute(cmd, |cmd: &str| {
//         Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
//     });
//     println!("{:?}", r2);
// }

// fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
//     exec.execute(cmd)
// }

fn main() {}
