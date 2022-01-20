struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)] //添加外部属性
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let mut user1 = User {
    //     email: String::from("fshouhai@gmail.com"),
    //     username: String::from("fushouhai"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("1821938264@qq.com");

    // let user2 = User {
    //     email: String::from("xxx@163.com"),
    //     ..user1
    // };

    // println!("user1 name: {}", user1.sign_in_count);
   
    // let name = "Jim Smith";
    // let first_name = &name[..4];


    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    
    let rect2 = Rectangle {
        width: 30, height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
      
        area(&rect2)
    );

    println!("{:?}", rect2)
}

fn area(recangle: &Rectangle) -> u32 {
    recangle.width * recangle.height
}
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
