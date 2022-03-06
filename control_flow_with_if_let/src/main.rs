/*
I asked a question about this section;
https://users.rust-lang.org/t/understanding-if-let-statement/72612/3

Now, I really know how this syntax works :)
*/

fn main() {
    println!("Hello, world!");

    let admin = is_admin(Some(Permissions {
        admin: true,
        user_id: 1,
    }));

    println!("Is admin {}", admin);
}


struct Permissions {
    user_id: i32,
    admin: bool,
}

// now i understand how this work.
// i'll try to explain this block by block
/*
    if -> this is normal if statement
        let -> we're saying that we'll create a variable
            Some(not_exist) -> create a variable called not_exist
                = -> not_exist variable is coming from
                    permission -> this variable hold a data

     Let's try this in human language.

     If permission variable has a value, create not_exist variable and use it if you want, if not exist? you can't use it use anything else
*/
fn is_admin(permission: Option<Permissions>) -> bool {
    if let Some(not_exist) = permission {
        return not_exist.admin;
    }

    false
}

fn is_odd(number: Option<i32>) -> bool {
    if let Some(x) = number {
        return x % 2 == 1
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_odd() {
        assert_eq!(is_odd(Some(2)), true, "Number is not odd");
    }

    #[test]
    fn test_is_odd() {
        assert_eq!(is_odd(Some(3)), true, "Number is not odd");
    }

    #[test]
    fn test_is_not_admin() {
        assert_eq!(is_admin(None), false, "This function expects return a boolean value");
    }

    #[test]
    fn test_is_admin() {
        assert_eq!(is_admin(Some(Permissions {
            admin: true,
            user_id: 1
        })), true, "This function expects return a boolean value");
    }
}
