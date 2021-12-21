// 总结
// 1. 值无法离开类型单独单独讨论。 类型是值在内存中的组织方式。一块作为不同类型下的不同的值，可能会有不同的表现。
// 2. 指针和引用都指向值的地址， 区别时引用只能解引用到引用得类型，并有很多使用限制。指针可以解指针到任何类型， 使用没有什么限制。
//    指针和引用使用时需要创建，不是随着值以来创建的。 指针和引用都存在栈上。
// 3. 运行时多态靠一个胖指针实现。 胖指针保存着值的地址以及trait的vtable。此时的值的类型已经被擦除了。
// 4. 并发是同时和多个任务打交道的能力（不是同时处理）。 并行是同时处理多个任务的手段。 并发是并行的基础。
// 5. 同步阻塞后面的逻辑， 异步不阻塞后面的逻辑。
// 6. 泛型编程就是在写代码的时候使用类型占位符，等到真正使用的时候再指定类型。写一个类型当一堆类型用， 提升代码的复用性。
// 7. struct类型的值包含里面所有类型的值。 enum类型的值时里面某个类型的值，由tag决定。
// 8. 某个enum类型的值时某一个子类型的值。子类型的值都可以作为某个enum类型的值。
// 9. enum match时时类型匹配加解构二合一。解构实际上就是一种取值的特殊写法。
// 10. 可有有可无的时候用Option<T>, 因为T必须有值。用Option<T>时，Some(T)类型接受有值，None类型接受没值得情况（用None类型的None值代替没有值得情况）。

// 问题
// 1. 有一个指向某个函数的指针，如果将其解引用成一个列表，然后往列表中插入一个元素，请问会发生什么？（对比不同语言，看看这种操作是否允许，如果允许会发生什么）
//    强类型语言不允许转换。 弱类型语言转后写失败。
// 2. 要构造一个数据结构 Shape，可以是 Rectangle、 Circle 或是 Triangle，这三种结构见如下代码。请问 Shape 类型该用什么数据结构实现？怎么实现？
// struct Rectangle {
//     a: f64,
//     b: f64,
//  }
//  struct Circle {
//    r: f64,
//  }
//  struct Triangle {
//    a: f64,
//    b: f64,
//    c: f64,
//  }

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

// 2. 对于上面的三种结构，如果我们要定义一个接口，可以计算周长和面积，怎么计算？
trait CalculatableShape {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}

impl CalculatableShape for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }

    fn circumference(&self) -> f64 {
        (self.a + self.b) * 2f64
    }
}

impl CalculatableShape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }

    fn circumference(&self) -> f64 {
        2f64 * 3.14 * self.r
    }
}

impl CalculatableShape for Triangle {
    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }

    fn circumference(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl CalculatableShape for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle(rectangle) => rectangle.area(),
            Shape::Circle(circle) => circle.area(),
            Shape::Triangle(triangle) => triangle.area(),
        }
    }

    fn circumference(&self) -> f64 {
        match self {
            Shape::Rectangle(rectangle) => rectangle.circumference(),
            Shape::Circle(circle) => circle.circumference(),
            Shape::Triangle(triangle) => triangle.circumference(),
        }
    }
}

fn main() {}
