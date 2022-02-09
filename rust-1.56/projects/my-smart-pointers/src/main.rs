use std::ops::Deref;

fn main() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(5);
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let name = MyBox::new(String::from("Rust"));
    hello(&(*name)[..]);
    // 二者等价
    hello(&name);
    // Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。 不是很理解具体过程！！！


}

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}