// use std::sync::{Mutex, Arc};
// use std::thread;
// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }
//     println!("Result: {}", *counter.lock().unwrap());
// }

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    print!("The secret number is {}\n", secret_number);

    print!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    print!("You guessed: {}", guess);
}
