fn main() {
    println!("Hello, world!");
}

fn variable_scope() -> String {

    // he is a man from kocaeli
    let name = "Ali";

    { // there is no Ali yet

        let name = "Ali GOREN"; // he is a man from istanbul
    } // we're leaving istanbul and back to the kocaeli

    // we'll see ali from kocaeli.

    return format!("{}", name);
}

fn string_type(surname: &str) -> String {
    let mut person = String::from("Ali ");

    person.push_str(surname);

    return person;
}

fn string_type_copy() -> String {
    let name_surname = String::from("Ali GOREN");
    let name_surname_copy = name_surname;

    //return name_surname;

    name_surname_copy
}

#[cfg(test)]
mod tests {
    use crate::{string_type, string_type_copy, variable_scope};

    #[test]
    fn test_string_type_copy() {
        assert_eq!(string_type_copy(), "Ali GOREN", "Who are you?");
    }

    #[test]
    fn test_string_type() {
        assert_eq!(string_type("GOREN"), "Ali GOREN", "Who is there?");
    }

    #[test]
    fn test_variable_scope() {

        assert_eq!(variable_scope(), "Ali", "Name value should be Ali");
    }
}