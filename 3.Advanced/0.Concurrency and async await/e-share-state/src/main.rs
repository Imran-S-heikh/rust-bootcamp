pub mod exercise_0;
pub mod exercise_1;

use std::sync::Mutex;

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Self {
        Database {
            connections: vec![],
        }
    }

    fn connect(&mut self, id: u32){
        self.connections.push(id)
    }
}

fn main() {
    let db = Mutex::new(Database::new());

    {
        let mut db = db.lock().unwrap();
        db.connect(32);
    }

    exercise_0::main();
    exercise_1::main();
}
