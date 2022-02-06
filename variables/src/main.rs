fn main() {
    println!("integer_value: {}", integer_value());
    println!("float_value: {}", float_value());
    println!("string_value: {}", string_value());
    println!("mutable_string_value: {}", mutable_string_value("Ali"));
    println!("mutable_float_value: {}", mutable_float_value(3));
    println!("mutable_integer_value: {}", mutable_integer_value(6));
}

fn integer_value() -> i32 {
    let x = 10;

    return x;
}

fn float_value() -> f64 {
    let x: f64 = 10.5;

    return x;
}

fn string_value() -> &'static str {
    let message = "Hello, world!";

    return message;
}

fn mutable_string_value(name: &str) -> String {
    let mut user_name = "Ali";

    if !name.is_empty() {
        user_name = name;
    }

    return format!("Hello, {}!", user_name);
}

fn mutable_float_value(number: i32) -> f64 {
    let mut integer_value: i32 = 1;
    let default_rate: f64 = 3.75;

    if number.is_positive() {
        integer_value = number;
    }

    return default_rate * integer_value as f64;
}

fn mutable_integer_value(number: i32) -> i32 {
    let mut default_value = 1;
    let default_rate = 2;

    if number.is_positive() {
        default_value = number;
    }

    return default_rate * default_value;
}

#[cfg(test)]
mod tests {
    use crate::{float_value, integer_value, mutable_float_value, mutable_integer_value, mutable_string_value, string_value};

    #[test]
    fn test_mutable_string() {
        let name: &str = "Ali";

        assert_eq!(mutable_string_value(name), "Hello, Ali!");
    }

    #[test]
    fn test_mutable_integer() {
        let number: i32 = 5;
        assert_eq!(mutable_integer_value(number), 10);
    }

    #[test]
    fn test_mutable_float() {
        let number: i32 = 2;
        assert_eq!(mutable_float_value(number), 7.5);
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer_value(), 10);
    }

    #[test]
    fn test_float() {
        assert_eq!(float_value(), 10.5);
    }

    #[test]
    fn test_string() {
        assert_eq!(string_value(), "Hello, world!");
    }
}