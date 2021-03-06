use std::env;
use std::process;

use mygrep::Config;

fn main() {
    // collect 可以被用来创建很多类型的集合，所以这里显式注明 args 的类型来指定我们需要一个字符串 vector。
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        // 标准库提供了 eprintln! 宏来打印到标准错误流
        eprintln!("{}\n", error);
        process::exit(1);
    });

    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);

    //     process::exit(1);
    // }

    mygrep::run(config).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
}
