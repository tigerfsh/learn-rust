// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };


// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));

// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//     }
// }
// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
//     let m = Message::Write(String::from("hello, world."));
//     m.call();
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn main() {
//     let val = value_in_cents(Coin::Penny);
//     println!("{}", val);
// }


// #[derive(Debug)] // 这样可以立刻看到州的名称
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alabama));
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(v) => Some(v+1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?}, {:?}", six, none);
// }

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //不做任何处理
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}