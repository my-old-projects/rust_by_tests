use rand::Rng;

fn guess_number(number: i32) -> bool {
    if !(1..101).contains(&number) {
        panic!("The number you enterest must be in valid ranges. Valid ranges are: 1-100");
    }

    let random_number: i32 = rand::thread_rng().gen_range(1..101);

    return random_number == number;
}

fn main() {
    println!("Pleas guess a number!");
}


#[cfg(test)]
mod tests {
    use std::any::type_name;
    use crate::guess_number;

    #[test]
    #[should_panic(expected = "The number you enterest must be in valid ranges")]
    fn is_number_in_valid_ranges() {
        guess_number(-1);
    }

    #[test]
    fn is_data_format_valid() {
        let result = guess_number(5);

        assert!((result | !result), "Test didn't passed. Result is {}", result);
    }

}