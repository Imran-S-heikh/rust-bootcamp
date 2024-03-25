use std::thread;

pub fn main() {
    // Fix the code to make it compile.

    let msg = "Hello from spawned thread".to_owned();
    let handle = thread::spawn(move || {
        println!("{msg}");
    });
    handle.join().unwrap();
}
