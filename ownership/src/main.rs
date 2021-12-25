use std::{cell::RefCell, rc::Rc};

// 总结
// 0. 给一个值赋值就是改变scope内值所在位置的内存。
// 1. Rust使用ownership机制管理内存。 一个值被一个scope own， 一个值同某一时刻只能被一个scope own。 当变量所在的scope结束之后，值被drop。 值关联的heap也会被drop。
// 2. 为了实现ownership的某一时刻只能被一个scope own， Rust默认采用move语义。 但是仅有move语义会导致编码的复杂性（move过去move回来）。Rust又设计了copy和borrow。
// 3. copy为了解决简单类型和引用不需要move的问题（因为这些类型不关联heap，不会导致double free）。 borrow是为了解决move来去的问题。
// 4. borrow而来的引用得声明周期不能超过值的声明周期。
// 5. Rust也支持一个值(heap)拥有多个owner（平等关系）(栈上的值， 可以在多个scope内， 也可以在一个scope内)。 通过引用计数管理。内存分配使用Box::leak()完成。
// 6. 堆上的值被栈上的值own 栈上的值最终被scope own
// 7. Rc/Arc own的值如果要想被改变就要具有内部可变性或者被具有内部可变性的结构包裹(通过只读引用拿到内部的可变引用)。 因为Rc/Arc只能去到只读的引用。
// 8. 每个值都都有一个或者多个owner,可能是值或者scope, 当一个值的owner被drop（或者scope完成）的时候当前值也会被drop。
// 9. 每个值的root owner都是都一定是scope， scope一定会结束，所以所有值都能被drop。Rust的自动内存回收就是scope结束引发的连锁反应。
// 10. 因为ownership的关系，Rust的默认语义必须是move。因为如果值被复制到多个owner的话，可能会出现use after free的问题。如果需要使用多个owner，
//     那么应该使用rc,而并不是简单的复制。rc能保证到0之后释放。
// 11. 因为move在有些地方用起来很不方便（多次使用一个值）。所以又增加了copy和borrow。copy适用于简单类型和并不可变引用，因为copy损耗很小，并且并不own东西，不会出现use after free.
//     borrow用于解决多次使用一个值的问题，并对ownership没有影响。 但是有使用限制，就是引用的生命周期必须小于值的生命周期。

// 思考
// 1. 堆上的值能引用栈上的值吗
// 可以。只要引用的声明周期不大于栈上值得声明周期。也就是堆上的值关联的栈上的值的生命周期小于等于被引用栈上的值的声明周期。
// 2. main() 函数传递给 find_pos() 函数的另一个参数 v，也会被移动吧？为什么图上并没有将其标灰？
// 因为v的类型时copy语义 两个scope own的是不同的值。

// 1. 上一讲我们在讲 Copy trait 时说到，可变引用没有实现 Copy trait。结合这一讲的内容，想想为什么？
// 实现copy就会在多个scope有多个mut ref 可能会造成内存安全问题
// 2.下面这段代码，如何修改才能使其编译通过，避免同时有只读引用和可变引用？

// 1. Rc的clone()是不可变引用，如何实现的count++
// 因为Cell具有内部可变性 是通过unsafe实现的
// fn main() {
//     let mut arr = vec![1, 2, 3];
//     // cache the last item
//     let last = arr.last().unwrap().clone();
//     arr.push(4);
//     // consume previously stored last item
//     println!("last: {:?}", last);
// }


#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }

    fn new_with_next(val: T, next: Rc<RefCell<Node<T>>>) -> Self {
        Self {
            val,
            next: Some(next),
        }
    }

    fn get_next(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.next.as_ref().map(|next| next.clone())
    }

    fn set_next(&mut self, next: Rc<RefCell<Node<T>>>) {
        self.next = Some(next)
    }
}

fn main() {
    let node1 = Node::new_with_next(1, Rc::new(RefCell::new(Node::new(2))));
    // option.as_ref Some(T) -> Some(&T)
    node1.get_next().as_ref().map(|next| {
        next.borrow_mut()
            .set_next(Rc::new(RefCell::new(Node::new(3))))
    });
    println!("{:?}", node1);
}
