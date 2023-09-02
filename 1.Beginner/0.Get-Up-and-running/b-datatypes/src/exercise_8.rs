// Destructure the `cat` tuple so that the println will work.

pub fn run() {

    let cat = ("Furry McFurson", 3.5);
    let (name,age) /* Your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}