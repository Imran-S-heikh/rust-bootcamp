// Make this program compile by replacing the variable type.

pub fn run() {
    let number_of_stars: u64;
    number_of_stars = 400_000_000_000; // The Milky Way has more stars than can fit in a 32-bit integer type!
    println!("There are about {} stars in the Milky Way galaxy!", number_of_stars);
}