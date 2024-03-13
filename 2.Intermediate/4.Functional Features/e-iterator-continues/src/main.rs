pub mod exercise_0;
pub mod exercise_1;
pub mod exercise_2;

struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;

    fn into_iter(self) -> Self::IntoIter {
        PersonIterator {
            values: vec![self.first_name, self.last_name, self.occupation],
        }
    }
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }

        Some(self.values.remove(0))
    }
}

fn main() {
    let person = Person {
        first_name: String::from("Imran"),
        last_name: String::from("Shaikh"),
        occupation: String::from("Softwere Engineer"),
    };

    for item in person {
        println!("{item}");
    }


    exercise_0::main();
    exercise_1::main();
    // exercise_2::main();
}
