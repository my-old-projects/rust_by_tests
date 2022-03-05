// TODO: I didn't understand this topic. I know how this section work.
// TODO: But I need to understand clearly.
// TODO: Check this document ../.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/book/ch06-03-if-let.html
// TODO: Make more practices :)

fn main() {
    println!("Hello, world!");

    let user_permission_id: Option<i32> = Some(1520);

    user_has_mod_permissions(user_permission_id);

}

// I think this section is not working as I expected from my side.
fn age_should_be_bigger_than_legal_age(_user_age: Option<i32>) -> String {
    let age: Option<i32> = Some(18);

    if let Some(_user_age) = age {
        return format!("Age is not verified {}", _user_age.to_string());
    }

    return format!("Age verified");
}

// This also works wrong.
fn user_has_mod_permissions(_permissions_id: Option<i32>) -> bool {
    let mod_permission_id: Option<i32> = Some(1521);

    println!("ID {}", _permissions_id.unwrap().to_string());

    if let Some(_permissions_id) = mod_permission_id {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_user_has_mod_permissions() {
        let user_permission_id: Option<i32> = Some(1520);
        assert_eq!(user_has_mod_permissions(user_permission_id), true, "User has no mod permissions");
    }

    #[test]
    fn test_for_legal_age() {
        let age: Option<i32> = Some(18);
        assert_eq!(age_should_be_bigger_than_legal_age(age), format!("Age is not verified {}", age.unwrap().to_string()));
    }
}