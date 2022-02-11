use std::ops::Deref;
use crate::List::{Cons, Nil};
use std::rc::Rc;

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

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(_c);

    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let my_name = String::from("fushouhai");
    let copy_name = Rc::new(my_name);
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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}