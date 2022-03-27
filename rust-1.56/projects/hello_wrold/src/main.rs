fn main() {
    let age = 18;
    let _address = &(age as f64);
    println!("address: {:p}", &(age as f64));

    println!("Hello, world!");
}
