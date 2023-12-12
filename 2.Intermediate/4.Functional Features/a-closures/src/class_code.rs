

struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

pub fn main() {
    let weak_password = "password123!";
    let validator = |username: &str, password: &str| {
        !username.is_empty()
            && !username.is_empty()
            && password.len() > 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            && password != weak_password
    };

    let creds = Credentials {
        username: String::from("admin"),
        password: String::from("password****"),
        validator,
    };

    println!("{}", creds.is_valid());
}
