fn first_function() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("{}", first_function());
}

#[cfg(test)]
mod tests {
    use crate::first_function;

    #[test]
    fn message_is_validate() {
        assert_eq!(first_function(), "Hello, world!");
    }
}