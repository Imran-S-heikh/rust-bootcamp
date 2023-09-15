// Make the code execute successfully by correctly implementing Message trait for Cat type.

trait Message {
    fn message(&self) -> String {
        "Default Message!".to_string()
    }
}

struct Fish;
struct Cat;
impl Message for Fish {}
impl Message for Cat {
    fn message(&self) -> String {
        String::from("Meow ðŸ±")
    }
}

pub fn main() {
    let fish = Fish;
    let cat = Cat;
    assert_eq!(String::from("Default Message!"), fish.message());
    assert_eq!(String::from("Meow ðŸ±"), cat.message());
}
