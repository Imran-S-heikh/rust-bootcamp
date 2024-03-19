use std::thread;

pub mod exercise_0;
pub mod exercise_1;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Spawn Thread: {}", i);
        }
    });

    for i in 0..10 {
        println!("Main Thread {}", i);
    }

    handle.join().unwrap();
    exercise_1::main();
}
