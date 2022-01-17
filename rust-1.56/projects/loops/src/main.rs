fn main() {
    let mut count = 0;
    loop {
        if count == 10 {
            break;
        }
        println!("{}", count);
        count += 1;

    }
    println!("Hello, world!");

    let mut count = 0;
    // 循环标签 loop label, break 'counting_up; 语句将退出外层循环。
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                println!("{}", "done");
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    let nums = [11, 22, 33, 44];
    let mut i = 0;
    while i < nums.len() {
        println!("{}", nums[i]);
        i += 1;
        
    }

    for item in nums.iter() {
        println!("{}", item);
    }

    for item in (1..4).rev() {
        println!("{}", item);
    }

    let mut a = 0;
    let mut b = 1;
    // while a < 100 {
    //     println!("{}", a);
    //     let c = a + b;
    //     a = b;
    //     b = c;
        
    // }

    for _ in 1..101 {
        println!("{}", a);

        let c = a +b; 
        a = b;
        b = c;
    }
}
