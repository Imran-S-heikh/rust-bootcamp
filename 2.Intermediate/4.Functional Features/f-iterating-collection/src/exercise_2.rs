// Fix the code to make it compile.

use std::collections::HashMap;

pub fn main() {
    // marks scored out of 50
    let mut marks = HashMap::from([("Harry", 40.0), ("Hermoine", 50.0), ("Ron", 35.5)]);
    // convert marks into percentage
    for (_, marks) in &mut marks {
        *marks = (*marks * 100.0) / 50.0;
    }
    marks
        .iter()
        .for_each(|(student, marks)| println!("{student} scored {marks}%"));
}
