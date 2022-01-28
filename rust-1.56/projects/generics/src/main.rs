struct Point<T> {
    x: T,
    y: T,
}

struct Diff<T, U> {
    x: T,
    y: U,
}

impl<T, U> Diff<T, U> {
    fn mixup<O, P> (self, other: Diff<O, P>) -> Diff<T, P>{
        Diff {
            x: self.x,
            y: other.y
        }
    }
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let integer = Point{x: 5, y: 10};
    let float = Point{x: 10.5, y: 11.5};
    println!("float.x = {}", float.x());


    let float = Diff{x: 10, y: 11.1};
    let char = Diff{x: 11, y: 100.9};

    let other = float.mixup(char);

}
