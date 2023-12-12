// Define the `double` closure & make the code execute successfully.

pub fn main() {
    let double = |num: i32| num * 2;
    assert_eq!(double(5), 10);
    assert_eq!(double(-10), -20);
}
