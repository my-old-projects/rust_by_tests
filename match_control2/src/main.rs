//TODO: This section requires more practice
fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
enum LoginType {
    Manager,
    Staff,
    Client
}

fn get_user_level(level_id: Option<i32>) -> Option<String> {
    match level_id {
        Some(level_id) => Some(format!("Authorized {}", level_id)),
        None => None // if we don't use none, this code will not be compiled
    }
}

fn get_login_permission_id(login_type: LoginType) -> i32 {
    match login_type {
        LoginType::Manager => 1,
        LoginType::Staff => 2,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_login_permission_for_staff() {
        assert_eq!(get_login_permission_id(LoginType::Staff), 2, "Permission should be defined for staff");
    }

    #[test]
    fn test_get_login_permission_with_zero() {
        assert_eq!(get_login_permission_id(LoginType::Client), 0, "Permission shouldn't be defined");
    }

    #[test]
    fn test_get_user_level_admin() {
        let level_id = Some(1);
        let level = get_user_level(level_id);

        assert_eq!(level.unwrap(), "Authorized 1", "levels didn't match");
    }

    #[test]
    fn test_get_user_level_none() {
        let level_id = None;
        let level = get_user_level(level_id);

        assert_eq!(level, None, "levels didn't match");
    }
}