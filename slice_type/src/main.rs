fn main() {
    println!("Hello, world!");
}

fn get_first_character(s: &String) -> char {
    let index: usize = 0;

    let character = s.as_bytes()[index];

    return character as char;
}

fn get_characters_after_given_index(index: usize, message: &String) -> &str {
    return &message[index..];
}

fn clean_string(s: &mut String) -> &str {
    s.clear();

    return s;
}

fn get_string_length(s: &String) -> usize {
    return s.len();
}

fn iterate_and_break(index: usize, message: &String) -> char {
    let bytes = message.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if i == index {
            return item as char;
        }
    }

    return bytes[0] as char;
}

fn first_character_of_name(index: usize, name: &String) -> char {
    return name.as_bytes()[index] as char;
}

fn get_second_character_of_name(index: usize, name: &String) -> char {
    return name.as_bytes()[index] as char;
}

#[cfg(test)]
mod tests {
    use crate::{
        clean_string, first_character_of_name, get_characters_after_given_index,
        get_first_character, get_second_character_of_name, get_string_length, iterate_and_break,
    };

    #[test]
    fn test_get_second_character_of_name() {
        let index: usize = 1;
        let name = &String::from("John");
        assert_eq!(
            get_second_character_of_name(index, &name),
            'o',
            "Chars didn't match"
        );
    }

    #[test]
    fn test_first_character_of_name() {
        let index: usize = 0;
        let name = &String::from("John");

        assert_eq!(
            first_character_of_name(index, &name),
            'J',
            "Chars didn't match"
        );
    }

    #[test]
    fn test_iterate_and_break() {
        let message = &String::from("Hello world!");

        assert_eq!(iterate_and_break(2, &message), 'l', "chars didn't match");
    }

    #[test]
    fn test_get_string_length() {
        let test_string = String::from("Hello, Ali");
        let length = get_string_length(&test_string);
        assert_eq!(
            length, 10,
            "You expected {} for length. But function returned {}",
            10, length
        );
    }

    #[test]
    fn test_clean_string() {
        let mut message = String::from("Hello, Ali");

        assert_eq!(clean_string(&mut message), "", "Results didn't match");
    }

    #[test]
    fn test_get_characters_after_given_index() {
        let message = String::from("Hello, Ali");

        assert_eq!(
            get_characters_after_given_index(2, &message),
            "llo, Ali",
            "Results didn't match"
        );
    }

    #[test]
    fn test_get_first_character() {
        let name = String::from("Ali");

        assert_eq!(get_first_character(&name), 'A', "Characters didn't match");
    }
}
