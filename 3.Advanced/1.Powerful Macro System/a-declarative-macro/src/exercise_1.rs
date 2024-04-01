// Everything seems correct, but the code does not compile. Maybe it has to do with the position of defining a macro.

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check this macro, Its working!");
    };
}

pub fn main() {
    my_macro!();
}
