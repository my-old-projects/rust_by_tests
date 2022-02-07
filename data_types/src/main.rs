fn main() {
    println!("Hello, world!");
}

fn be_careful_for_overflow(value: i8) -> i8 {
    let number: i8 = 125;

    number + value
}

fn be_careful_for_index_overflow(index: usize) -> i32 {
    let arr = [1, 2, 3 ,4 ,5];

    return arr[index];
}

#[cfg(test)]
mod tests {
    use crate::{be_careful_for_index_overflow, be_careful_for_overflow};

    #[test]
    fn should_be_i8() {
        let number1: i8 = 25;
        let number2: i8 = 25;
        let result: i8 = number1 + number2;
        assert_eq!(result, 50, "Values aren't the same");
    }

    #[test]
    fn should_be_i32() {
        let number1: i32 = 25;
        let number2: i32 = 25;
        let result: i32 = number1 + number2;
        assert_eq!(result, 50, "Values aren't the same");
    }

    #[test]
    #[should_panic]
    fn maybe_integer_overflow() {
        let value: i8 = 10;

        be_careful_for_overflow(value);
    }

    #[test]
    fn should_be_arch() {
        let number1: isize = 34512521521;
        let number2: isize = 1;
        let result = number1 + number2;

        assert_eq!(result, 34512521522, "Values are incorrect");
    }

    #[test]
    fn should_be_f64() {
        let number1 = 3.5;
        let number2 = 1;
        let result = number1 + number2 as f64;

        assert_eq!(result, 4.5, "Values are incorrect");
    }

    #[test]
    fn should_be_true() {
        let number1 = 1;
        let number2 = 1;
        let result = number1 == number2;

        assert_eq!(result, true, "Values are incorrect");
    }

    #[test]
    fn should_output_emoji() {
        let emoji = '😻';

        assert_eq!(emoji, '😻', "Values are not the same");
    }

    #[test]
    fn tuple_values_should_be_matched() {
        let tup: (i32, bool) = (500, true);

        assert_eq!(tup.1, true, "Values are not matched {}", tup.1);

    }

    #[test]
    #[should_panic]
    fn array_can_be_panic_because_of_overflow() {
        let index: usize = 6;
        be_careful_for_index_overflow(index);
    }
}