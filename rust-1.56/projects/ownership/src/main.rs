// fn main() {
//     let mut s = String::from("Hello");

//     s.push_str(" Beijing");
//     println!("{}", s);

//     takes_ownership(s);

//     let x = 5;
//     makes_copy(x);

//     println!("{}", x);

// }

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);

    // let r1 = &mut s;
    // println!("{}, {}", r1, r1);

    let w = first_word(&s[..]);
    println!("{}", w);

    let s1 = &s[0..2];
    println!("{}", s1);

    let s2 = "Welcome to Beijing";
    println!("{}", s2);

}

fn first_word(s: &str) -> &str {
    println!("String: {}", s);

    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
