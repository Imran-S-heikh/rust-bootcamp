// Implement the add method for Pair<i32> type.

struct Pair<T>(T, T);

impl Pair<i32> {
    fn add(&self) -> i32 {
        self.0 + self.1
    }
}

pub fn main() {
    let p1 = Pair(10, 23);
    let addition = p1.add();
    assert_eq!(addition, 33);
}
