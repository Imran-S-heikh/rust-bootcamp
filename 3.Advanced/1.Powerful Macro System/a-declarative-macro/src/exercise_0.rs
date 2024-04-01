// Fix the code to make it compile.

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

pub fn main() {
    my_macro!();
}
