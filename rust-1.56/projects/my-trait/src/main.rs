use my_lib::{Tweet, NewsArticle};
use my_lib::Summary;

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn largest_v2<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for item in list.iter() {
        if *item > *max {
            max = item;
        }
       
    }
    max
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you ..."),
        reply: false,
        retweet: false,
    };

    // println!("{}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    // println!("New article available! {}", article.summarize());

    notify(&tweet);
 
    notify(&article);
    println!("{}", tweet.summarize());

    let numbers = vec![10, 20, 30, 15];
    println!("{}", largest(&numbers));
    println!("{}", largest_v2(&numbers));


}
