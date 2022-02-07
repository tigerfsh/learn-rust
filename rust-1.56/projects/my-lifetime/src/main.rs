fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_v2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest_v3<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("hello world.");
    result.as_str()

}

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // // 借用检查器
    // println!("{}", r);

        let string1 = String::from("abc");
        let string2 = "xyz";

        let res = longest(&string1, string2);

        println!("The longest string is {}", res);


}
