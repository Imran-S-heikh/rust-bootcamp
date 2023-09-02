// Fix the code so that it compiles. Modify only one statement.

pub fn run() {
    let mut my_str = String::from("Example");
    let mut temp;
    while my_str.len() > 0 {
        temp = my_str.clone();
        println!("Length of temporary string is: {}", temp.len());
        my_str.pop();
    }
}