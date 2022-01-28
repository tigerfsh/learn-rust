use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // let v = vec![11, 22, 33];
    // println!("{}", v[0]);

    // let f = File::open("a.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("a.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("{}", e),
    //         },
    //         other => panic!("{:?}", other),
    //     },
    // };

    // // 等价写法
    // let f = File::open("a.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("a.txt").unwrap_or_else(|error| {
    //             panic!("{:?}", error);
    //         })
    //     } else {
    //         panic!("{:?}", error);
    //     }
    // });

    // let f = File::open("b.txt").unwrap();

    // // 等价写法
    // let f = match File::open("b.txt") {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("{:?}", error);
    //     }
    // };

    // let f = File::open("b.txt").expect("b.txt not exists.");

    let s = read_username_from_file().unwrap();
    println!("{}", s);

    let nums = [11, 22, 100, 55];
    let v = largest(&nums);
    println!("{}", v);

    let nums = vec![100, 101, 200, 98];
    let v = largest(&nums);
    println!("{}", v);

}

fn largest(list: &[i32]) -> i32 {
    let mut largest_num = list[0];
    for &item in list {
        if item > largest_num {
            largest_num = item;
        }
    }
    largest_num
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // let mut s = String::new();
    // let f = File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    
    fs::read_to_string("hello.txt")

}