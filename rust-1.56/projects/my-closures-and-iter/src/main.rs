
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    print!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(num.into()));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    generate_workout(1, 15);

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got {}", val);
    }

    let v2 = vec![11, 22, 33];
    let v2_iter = v2.into_iter();
    for val in v2_iter {
        println!("{}", val);
    }
    // println!("{:?}", v2); // moved

    let mut v3 = vec![101, 202, 303];
    let v3_iter = v3.iter_mut();
    for val in v3_iter {
        println!("{}", val);
        *val = *val + 100;
    }

    let v3_iter = v3.iter_mut();
    for val in v3_iter {
        println!("{}", val);
        *val = *val + 100;
    }


    
    
    // let mut v1_iter = v1.iter();
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());

    let name = String::from("fushouhai");
    let call_me = || println!("{}", name);

    call_me();

    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());

    let result = Counter::new().zip(Counter::new().skip(0)).map(|(a, b)| a * b).filter(|a| {a % 2 == 0});
    for val in result {
        println!("{:?}", val);
    }

}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }

        
    }
}