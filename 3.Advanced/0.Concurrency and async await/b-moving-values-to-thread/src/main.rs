pub mod exercise_0;

use std::thread;



fn main() {
    let text = String::from("Hello world");

    let handle = thread::spawn(move || {
        println!("{text}");
    });

    exercise_0::main();
}
