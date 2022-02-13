fn main() {
    println!("Hello, world!");
}

fn calculate_string_length(s: &String) -> usize {
    // borrowing happens here by reference using &
    return s.len();
}

fn reference_str() -> String {
    // we create a string here
    let my_name = String::from("Ali GÖREN");

    // we don't move ownership here. instead, we just gave it for a while.
    calculate_string_length(&my_name);

    // we can still use this variable here. because we don't move ownership.
    // we didn't have ownership for a while then we get it back
    return my_name;
}

fn change_str(data: &mut String) {
    let extra_data = "Welcome";

    data.push_str(extra_data)
}

fn mutate_borrowed_item() -> String {
    let mut message = String::from("Hello Ali. ");

    change_str(&mut message);

    return message;
}

// i'm creating this methods again
// because i didn't understand it correctly
// so i have to practice
fn change_str_two(name: &mut String) {
    let surname = "GOREN";

    name.push_str(surname);
}

fn add_surname() -> String {
    let mut name = String::from("Ali ");

    change_str_two(&mut name);

    return name;
}

fn get_len(name: &String) -> usize {
    return name.len();
}

fn get_size_of_string_with_immutable_param() -> usize {
    let name = String::from("Ali GOREN");

    let string_length = get_len(&name);

    return string_length;
}

fn take_pen_from_your_friend_for_a_while(pen: &String) {
    println!("I borrowed this pen: {}", pen);
}

fn borrow_your_pen_to_your_classmate() -> String {
    let pen = String::from("Faber-Castell");

    take_pen_from_your_friend_for_a_while(&pen);

    return format!("Thanks. I get back my {}", pen);
}

fn this_program_wont_compile_because_of_this_function() -> String {
    // let say this is a space station called STATION 1
    let mut name = String::from("NASA");

    let name1 = &mut name; // this variable point to STATION 1 and can mutate it
    let name2 = &mut name; // this variable also point to STATION 1 and try to mutate it

    // so at this point, this program won't compile.
    // because data race can happen here.
    // what would happen if this program compile

    /*
        name1.push_str("ISS"); will change the original one
        name2 still pointing to the old one so, this could be a potential data race
    */

    return format!("Name1: {}, Name2: {}", name1, name2);
}

#[cfg(test)]
mod tests {
    use crate::{add_surname, borrow_your_pen_to_your_classmate, get_size_of_string_with_immutable_param, mutate_borrowed_item, reference_str, this_program_wont_compile_because_of_this_function};

    #[test]
    fn test_this_program_wont_compile_because_of_this_function() {
        assert_eq!(this_program_wont_compile_because_of_this_function(), "Name1: NASA, Name2: NASA");
    }

    #[test]
    fn test_borrow_your_pen_to_your_classmate() {
        assert_eq!(
            borrow_your_pen_to_your_classmate(),
            "Thanks. I get back my Faber-Castell",
            "Strings didn't match"
        );
    }

    #[test]
    fn test_get_size_of_string_with_immutable_param() {
        assert_eq!(
            get_size_of_string_with_immutable_param(),
            9,
            "Length values didn't match"
        );
    }

    #[test]
    fn test_add_surname() {
        assert_eq!(add_surname(), "Ali GOREN", "Strings didn't match");
    }

    #[test]
    fn test_mutated_reference() {
        assert_eq!(
            mutate_borrowed_item(),
            "Hello Ali. Welcome",
            "String didn't match"
        );
    }

    #[test]
    fn test_reference() {
        assert_eq!(reference_str(), "Ali GÖREN", "Something happened!");
    }
}
