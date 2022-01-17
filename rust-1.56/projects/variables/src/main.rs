fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }

    // println!("The value of x is: {}", x);

    // let spaces = "    ";
    // let spaces = spaces.len();

    // let m = -10.25;
    // println!("{}", m);

    // let res: u32 = 2/3;
    // println!("{}", res);

    // let tuple = (100, 200.25, true, 'A', "A");
    // println!("{:?}", tuple);

    // let num = [1, 2, 3];
    // println!("{:?}", num);

    another_function(5);
    println!("{}", five());
    println!("{}", plus_one(5));

}

fn plus_one(m: i64) -> i64 {
    m +1
}

fn five() -> u64 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
