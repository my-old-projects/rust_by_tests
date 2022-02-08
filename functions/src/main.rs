fn main() {
    println!("Hello, world!");

    hello_rust();

    hey_this_function_will_panic();
}

fn hello_rust() {
    println!("Hi there. I'm ali :)");
}

fn formatted_fn(age: i32) -> String {

    return format!("I'm {} yo", age);
}

fn scope_value_in_funtion(number: i32) -> i32 {

    // This is a scope example. So we can say my_age never exists actually
    // :P not like that but actual body don't know :)
    let age = {
        let my_age = 20;

        my_age + number
    };

    return age;
}

fn a_float_function_without_return_keyword() -> f64 {
    // the last expressions also have a meaning like "this should be returned"
    // if you add a semicolon to end of the statement, this code will not be compiled.
    // if you want to add semicolon, you need to explicit return way: return 12.5;
    // explicit return way required semicolon. you have to use semicolons
    12.5
}

fn hey_this_function_will_panic() {
    panic!("Even if nothing happened, something definitely happened");
}

#[cfg(test)]
mod tests {
    use crate::{a_float_function_without_return_keyword, formatted_fn, hey_this_function_will_panic, scope_value_in_funtion};

    #[test]
    fn function_should_return_formatted_string() {
        let my_age = 29;

        assert_eq!(formatted_fn(my_age), "I'm 29 yo", "string returned than expected");
    }

    #[test]
    fn function_return_value_from_scope() {
        assert_eq!(scope_value_in_funtion(9), 29, "Values aren't matched properly");
    }

    #[test]
    fn function_return_value_without_return_keyword() {
        assert_eq!(a_float_function_without_return_keyword(), 12.5, "Value seems to be wrong");
    }

    #[test]
    #[should_panic]
    fn a_function_can_panic() {
        hey_this_function_will_panic();
    }
}