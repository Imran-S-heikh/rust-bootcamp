pub mod  exercise_0;
pub mod  exercise_1;
pub mod  exercise_2;

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

fn get_default_creds<T>(f: T) -> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    Credentials {
        username: String::from("Hello"),
        password: String::from("pasword"),
        validator: f,
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, pass: &str| pass.len() >= min_len && pass.contains(['!', '@', '#']))
    } else {
        Box::new(move |_: &str, pass: &str| pass.len() >= min_len)
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

    let pass_validator = get_password_validator(10, true);

    let default_creds = get_default_creds(pass_validator);

    println!("{}",default_creds.is_valid());


    exercise_0::main();
    exercise_1::main();
    exercise_2::main();
}
