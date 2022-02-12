extern crate core;

use std::fmt::format;

fn main() {
    println!("Hello, world!");
}

fn if_else_condition(age: i32) -> String {
    if age < 18 {
        return format!("You can't signup to the system. Your age: {}", age);
    }

    return format!(
        "Congratulations. You're part of this community now. Your age: {}",
        age
    );
}

fn break_loop() -> i32 {
    let mut number = 0;

    loop {
        if number == 5 {
            break;
        }

        number += 1;
    }

    return number;
}

fn return_value_by_loop() -> i32 {
    let mut number = 0;

    return loop {
        if number == 18 {
            break number;
        }

        number += 1;
    };
}

fn return_value_by_while() -> i32 {
    let mut number = 0;

    while number < 20 {
        number += 1;
    }

    return number;
}

fn return_value_by_breaking_while_loop() -> i32 {
    let mut number = 0;

    while number < 20 {
        if number == 10 {
            break;
        }

        number += 1;
    }

    return number;
}

fn get_number_from_an_array(user_index: usize) -> i32 {
    let mut index = 0;
    let mut number = 0;
    let numbers = [1, 20, 40, 50, 89];

    while numbers.len() > 0 {
        if index == user_index {
            number = numbers[user_index];

            break;
        }

        index += 1;
    }

    return number;
}

fn get_an_item_from_array_with_overflow(index: usize) -> i32 {
    let numbers = [10, 20, 30, 40];

    if index > numbers.len() {
        panic!("You try to access an undefined item");
    }

    return numbers[index];
}

fn using_if_with_let(age: i32) -> bool {
    let condition = if age >= 18 { true } else { false };

    return condition;
}

#[cfg(test)]
mod tests {
    use crate::{
        break_loop, get_an_item_from_array_with_overflow, get_number_from_an_array,
        if_else_condition, return_value_by_breaking_while_loop, return_value_by_loop,
        return_value_by_while, using_if_with_let,
    };

    #[test]
    fn age_should_accepted() {
        assert!(using_if_with_let(18));
    }

    #[test]
    #[should_panic]
    fn array_should_panic_because_overflow_here() {
        get_an_item_from_array_with_overflow(10);
    }

    #[test]
    fn user_should_not_signup_to_the_system() {
        let age = 13;
        let result = if_else_condition(age);
        let expected_result = format!("You can't signup to the system. Your age: {}", age);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn user_should_signup_to_the_system() {
        let age = 18;
        let result = if_else_condition(age);
        let expected_result = format!(
            "Congratulations. You're part of this community now. Your age: {}",
            age
        );

        assert_eq!(result, expected_result);
    }

    #[test]
    fn loop_should_break_when_number_is_five() {
        assert_eq!(break_loop(), 5, "Loop didn't stop. There is a problem :/");
    }

    #[test]
    fn get_value_by_returning_loop_value() {
        assert_eq!(return_value_by_loop(), 18, "Numbers didn't matched");
    }

    #[test]
    fn get_value_from_while_loop() {
        assert_eq!(return_value_by_while(), 20, "Numbers didn't matched");
    }

    #[test]
    fn break_while_loop() {
        assert_eq!(
            return_value_by_breaking_while_loop(),
            10,
            "Numbers didn't matched"
        );
    }

    #[test]
    fn get_number_from_array_by_breaking_loop() {
        assert_eq!(get_number_from_an_array(3), 50, "Numbers didn't match");
    }
}
