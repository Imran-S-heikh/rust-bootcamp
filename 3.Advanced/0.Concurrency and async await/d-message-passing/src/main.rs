pub mod exercise_0;
pub mod exercise_1;


use std::{sync::mpsc, thread};

fn main() {
    let messages = vec![
        String::from("dlroW olleH"),
        String::from("surT eW tsuR nI"),
        String::from("!ytsuR teG s'teL"),
        String::from("!tsuB ro tsuR"),
    ];

    let (tx, rx) = mpsc::channel();

    for message in messages {
        let tx_cloned = tx.clone();

        thread::spawn(move || {
            let reversed: String = message.chars().rev().collect();
            tx_cloned.send(reversed).unwrap();
        });
    }

    drop(tx);
    
    for message in rx {
        println!("{message}")
    }

    exercise_0::main();
    exercise_1::main();
}
