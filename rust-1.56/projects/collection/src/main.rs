fn main() {
    // let v : Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    v.push(100);
    v.push(200);

    println!("{:?}", v);

    // 两种读取集合的方式， &[]：超出范围，触发panic; get方法，超出范围会返回None，根据自己的业务场景进行选择
    let third = &v[2];
    let third_again = &v[4];
    println!("{}, {}", third, third_again);

    match v.get(5) {
        Some(v) => println!("{}", v),
        None => println!("It's none."),
    }

    let _r = &v[3];
    v.push(500); // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // println!("{:?}", r);

    for i in &v {
        println!("{}", i);
    }

    for j in &mut v {
        println!("{}", j);
        *j += 10; //解引用
    }

    for i in &v {
        println!("{}", i);
    }

   
    assert_eq!(v.pop(), Some(510));

    let mut s = "Welcome to 2022.".to_string();

    s.push_str(" YiQi. ");
    s.push('A');
    println!("{}", s);

    
    let apple = String::from("apple ");
    let orange = String::from("orange");
    let banana = String::from("banana");

    let fruit = format!("{}, {}, {}", apple, orange, banana);
    println!("{}, {}", fruit, apple);
    
    let len = String::from("北京欢迎你").len();
    println!("len: {}", len);

    
    // Unicode标量值
    for c in "नमस्ते".chars() {
        println!("char: {}", c);
    }

    //原始的字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for c in "你好北京".chars() {
        println!("char: {}", c);
    }



    // 从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能。crates.io 上有些提供这样功能的 crate。

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 100);
    scores.insert(String::from("Blue"), 50);

    for (k, v) in &scores {
        println!("k1: {}, v1: {}", k, v);
    }
    // 覆盖旧值
    scores.insert(String::from("Blue"), 150);
    for (k, v) in &scores {
        println!("k2: {}, v2: {}", k, v);
    }

    // 在没有对应值时插入
    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Yellow")).or_insert(200);
    for (k, v) in &scores {
        println!("k3: {}, v3: {}", k, v);
    }

    // 根据旧值
    let mut cache = HashMap::new();
    let text = "AA BB AA BB CC";
    for word in text.split_whitespace() {
        let count = cache.entry(word).or_insert(0);
        *count += 1;
    }
    println!("cache: {:?}", cache);

    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let field_name = String::from("Favorite color");
    match map.get(&field_name) {
        Some(v) => println!("{}", v),
        None => {},
    }

    //if let 是match的一个语法糖
    if let Some(v) = map.get(&field_name) {
        println!("{}", v);
    } else {
        println!("just None.");
    }

    // &map，避免所有权转移到for loop
    for (k, v) in &map {
        println!("key: {}, value: {}", k, v);
    }

}
