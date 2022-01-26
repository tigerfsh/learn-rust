mod front_of_house {
    fn call() {
        println!("A function named call.");
    }
    pub mod hosting {
        use super::testing;
        use super::super::front_of_house;

        pub fn add_to_waitlist() {
            front_of_house::call(); //子模块调动父模块，可以直接调用
            testing::testing(); // 父模块调用子模块的项（从front_of_house中调子模块testing的函数testing），必须声明为公有的，使用pub关键字
        }
    }

    mod testing {
        pub fn testing() {
            println!("I am testing function.");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); //名为create的隐藏模块

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}